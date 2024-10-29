import sys 
import signal

from os.path     import isfile, join, dirname
from argparse    import ArgumentParser
from mega.errors import RequestError
from random      import randint
from os          import remove

from xpmlib.PasswordManagement import GetPassword, ModifyPassword, GeneratePassword, SavePassword
from xpmlib.Display            import DisplayPassword, DisplayError, DisplaySaved, DisplayKey
from xpmlib.FileEncryption     import CheckFile, Encrytion, Decryption
from xpmlib.File               import GetPath, FileReader, WriteFile
from xpmlib.Tar                import Tar, TarExtract
from xpmlib.Upload             import MegaUploader
from xpmlib.Color              import Color
from xpmlib.Key                import Key
from xpmlib.Error              import *

class Main:
    PASSWORD_FILE_NAME: str = '.password.json'
    TOOL_PATH: str =  dirname(__file__)
    DATABASE_PATH: str = join( TOOL_PATH, 'database' )
    PASSWORD_PATH: str = join( DATABASE_PATH, PASSWORD_FILE_NAME )
    PASSWORD_EN_PATH: str = f"{PASSWORD_PATH}.x"

    def __init__(self , args) -> None:
        signal.signal(signal.SIGINT, self.exit_handler)
        self.__args = args
        self.__key = None
        self.__constant_number = None
    # The Runner..
    def run( self ) -> None:
        # Password Management:
        if self.__args.password_management: self.password_management()
        # File Management: 
        elif self.__args.file_management: self.file_management()
        # Backup Management:
        elif self.__args.backup_management: self.backup_management()
        # Missing subtool
        else: DisplayError.log('Set one of [`-pm`, `-fm`] or run with `--help`!!')
        
    # Password Management
    def password_management( self ) -> None:
        self.database_stat(True)
        # Generate Password - Save password.
        if self.__args.length:
            try:
                password: str = GeneratePassword(self.__args.length).generate()
                if self.__args.save:
                    name: str = input('Enter name for the password: ')
                    # read the database
                    data: bytes = FileReader(self.PASSWORD_PATH).read()
                    # Set the Update to the Data
                    data = SavePassword( name=name, password=password, data=data).save()
                    # write the new data to the database
                    WriteFile( path=self.PASSWORD_PATH, data=data ).write()
                DisplayPassword.log(password)
            except FoundError as error:
                DisplayError.log(error)
                if self.__key: self.database_stat(False)
                sys.exit(0)
            except Exception as error:
                DisplayError.log(error)
        # Diplay Password
        if self.__args.show and (self.__args.table or self.__args.natural) :
            try:
                data: bytes = FileReader(self.PASSWORD_PATH).read()
                passwords: dict = GetPassword( data= data ).get_passwords()
                display = DisplayPassword( passwords= passwords )
                if self.__args.table:
                    display.table()
                elif self.__args.natural:
                    display.natural()
            except Exception as error:
                DisplayError.log(error)
        elif self.__args.show :
            DisplayError.log("Error: Missing `--table` or `--natural` !!")
        # Update Password info.
        if self.__args.update_password:
            try:
                name: str = input("Enter Password Name: ")
                data: bytes = FileReader(self.PASSWORD_PATH).read()
                GetPassword(data=data, name=name).get_password() # if name not in database -> NotFoundError
                print("---- Update ----")
                print("1. Set new Name.")
                print("2. Set new Password.")
                print("3. Set new Password and Name.")
                user_input: int = int(input("( 1<= x <=3 ): ")) # if not a number -> ValueError
                modfiy = ModifyPassword(name=name, data=data)
                new_password: str = None if not self.__args.length else GeneratePassword(self.__args.length).generate()
                if user_input == 1:
                    new_name: str = input("Enter the new name: ")
                    data = modfiy.update(choice=1, new_name=new_name)
                elif user_input == 2:
                    if not new_password: 
                        new_password = input("Enter the new password: ")
                    data = modfiy.update(
                        choice=2,
                        new_password= new_password
                    )
                elif user_input == 3 :
                    new_name: str = input("Enter the new name: ")
                    if not new_password: 
                        new_password = input("Enter the new password: ")
                    data = modfiy.update(
                        choice=3,
                        new_name= new_name,
                        new_password= new_password
                    )
                else: raise ValueError
                WriteFile(
                    path=self.PASSWORD_PATH,
                    data= data
                ).write()
                print( f"{Color.GREEN}Password Updated Successfully{Color.ORG}" )
            except ValueError:
                DisplayError.log("Error Input: Enter a Number in range ( 1<= x <=3 )!")
            except Exception as error:
                DisplayError.log(error)
        # Delete Password.
        if self.__args.delete_password:
            try:
                name: str = input("Enter the name: ")
                data: bytes = FileReader(self.PASSWORD_PATH).read()
                GetPassword(name= name, data= data).get_password() # if name not in database -> NotFoundError
                if input("Are you sure you want to delete [yes/No]: ").lower()[0] != 'y':
                    print( f'{Color.GREEN}Password Not Deleted...{Color.ORG}' )
                    self.exit_handler(None, None)
                modfiy = ModifyPassword( name= name, data= data )
                data = modfiy.delete()
                WriteFile(path= self.PASSWORD_PATH, data= data).write()
            except Exception as error:
                DisplayError.log(error)
            ...
        # Find Password
        if self.__args.find_password:
            try:
                name: str = input("Enter the name: ")
                data: bytes = FileReader(self.PASSWORD_PATH).read()
                password: str = GetPassword(data= data, name= name).get_password()
                DisplayPassword( passwords= { name: password } ).natural()
            except Exception as error:
                DisplayError.log(error)
        self.exit_handler(None, None)
    # File Management
    def file_management( self ) -> None:
        # File
        if self.__args.encrypt_file or self.__args.decrypt_file:
            try:
                file_path: str = input("Enter the file path: ")
                if not isfile(file_path): raise FileNotFoundError(f"`{file_path}` does not exist!")
                remove_file: bool = True if input('Do you want to remove the original file [yes/No]: ').lower()[0] == 'y' else False
                # Encrypt file
                if self.__args.encrypt_file:
                    if input('Did you have a key [yes/No]: ').lower()[0] == 'y':
                        self.set_key()
                    self.__key = Encrytion(
                        paths=[file_path],
                        key= self.__key,
                        remove_files= remove_file
                    ).encrypt()
                    self.get_key()
                # Decrypt file
                else:
                    self.set_key()
                    Decryption(
                        paths= [file_path],
                        key= self.__key,
                        remove_files= remove_file
                    ).decrypt()
                    self.get_key()
            except Exception as error:
                DisplayError.log(error)
        # Folder
        elif self.__args.encrypt_folder or self.__args.decrypt_folder:
            try:
                folder_path: str = input('Enter the folder path: ')
                paths: list[str] = GetPath( folder_path ).get_all_files_paths() # if not a dir -> NotADirectoryError
                remove_files: bool = True if input('Do you want to remove the original file [yes/No]: ').lower()[0] == 'y' else False
                if self.__args.encrypt_folder:
                    key: str = None
                    if input('Did you have a key [yes/No]: ').lower()[0] == 'y':
                        self.set_key()
                    self.__key = Encrytion(
                        paths= paths,
                        key= self.__key,
                        remove_files= remove_files
                    ).encrypt()
                    self.get_key()
                elif self.__args.decrypt_folder:
                    self.set_key()
                    Decryption(
                        paths= paths,
                        key= self.__key,
                        remove_files= remove_files
                    ).decrypt()
                    self.get_key()
            except Exception as error:
                DisplayError.log(error)
        # Missing value
        else:
            DisplayError.log('Set one of [ `-ef`, `-df`, `-efs`, `-dfs` ] or run with `--help`!!')
        # End of File Management
    # Backup Management
    def backup_management( self ) -> None:
        # Password Database backup
        if self.__args.password_database:
            try:
                isdatabaseenctypted = CheckFile( self.DATABASE_PATH ).isdatabaseencryption( self.PASSWORD_FILE_NAME )
                if not isdatabaseenctypted : raise ValueError('Database is not encrypted use `-pm` to encrypt the database!')
                # Local Backup
                if self.__args.local_backup:
                    save_path: str = input( "Enter the save path: " )
                    outfile_path: str = Tar(
                        folder_path= self.DATABASE_PATH,
                        save_path= save_path
                    ).folder()
                    DisplaySaved.log(outfile_path)
                # Mega Backup
                elif self.__args.mega_backup:
                    outpath: str = Tar(
                        folder_path= self.DATABASE_PATH,
                        save_path= self.DATABASE_PATH
                    ).folder()
                    try: 
                        upload = MegaUploader( 
                            file_path= outpath,
                            email= input('Enter your email on mega: '),
                            password= input('Enter your password: ')
                        ).upload_file()
                    except RequestError:
                        DisplayError.log('Login Error: Check you email or password!')
                    except Exception:
                        DisplayError.log('Something went wrong. Make sure you are connected to the network!')
                    finally:
                        remove(outpath)
                else: DisplayError.log( 'Missing one of [ `-lb`, `-mb` ] run with `--help`!' )
            except Exception as error:
                DisplayError.log(error)
        # Folder Backup
        elif self.__args.folder_backup:
            try:
                # Local Backup:
                if self.__args.local_backup:
                    folder_path: str = input( "Enter the folder path: ") 
                    save_path: str = input( "Enter the save path: " )
                    if input('Do you have a key [yes/No]: ').lower()[0] == 'y':
                        self.set_key()
                    outfile_path: str = Tar(
                        folder_path= folder_path,
                        save_path= save_path
                    ).folder()
                    try:
                        self.__key = Encrytion(paths= [outfile_path], key= self.__key, remove_files= True ).encrypt()
                        self.get_key()
                    except:
                        remove(outfile_path)
                    DisplaySaved.log( outfile_path )
                # Mega Backup:
                elif self.__args.mega_backup:
                    folder_path: str = input("Enter the folder path: ")
                    if input('Do you have a key [yes/No]: ').lower()[0] == 'y':
                        # self.__key = input('Enter the key: ').encode()
                        self.set_key()
                    outfile_path: str = Tar(
                        folder_path= folder_path,
                        save_path= self.DATABASE_PATH
                    ).folder()
                    try:
                        self.__key = Encrytion( 
                            paths= [outfile_path],
                            key= self.__key,
                            remove_files= True
                        ).encrypt()
                        self.get_key()
                        upload = MegaUploader( 
                            file_path= f"{outfile_path}.x",
                            email= input('Enter your email on mega: '),
                            password= input('Enter your password: ')
                        ).upload_file()
                    except RequestError:
                        DisplayError.log('Login Error: Check you email or password!')
                    except Exception:
                        DisplayError.log('Something went wrong!')
                else: DisplayError.log( 'Missing one of [ `-lb`, `-mb` ] run with `--help`!' )
            except Exception as error:
                DisplayError.log(error)
        # Recovery Password Database Backup
        elif self.__args.password_recovery:
            file_path: str = input("Enter file path: ")
            try:
                tar = TarExtract( file_path= file_path, extract_path= self.TOOL_PATH)
                print('All current data will be replaced with the recoverd data!')
                if input('Are you sure about the recovery [yes/No]: ').lower()[0] != 'y' :
                    print('I will Not..')
                    return
                tar.extract()
            except Exception as error:
                DisplayError.log(error)
        # Recovery Folder Backup
        elif self.__args.folder_recovery:
            path: str = input('Enter the file path: ')
            self.set_key()
            save_path: str = input("Enter the save path: ")
            try:
                file_path: str =  Decryption(paths=[path], key= self.__key ).decrypt()
                TarExtract(file_path= file_path, extract_path= save_path).extract()
                remove(file_path)
            except Exception as error:
                DisplayError.log(error)
        else: DisplayError.log(  'Missing one of [`-pdb`, `-fb`] or run with `--help`!'  )

    # Encrypt and Deecrypt the database
    def database_stat(self, decrypt:bool=False) -> None:
        open_data_base: bool = ( 
            self.__args.save or 
            self.__args.update_password or 
            self.__args.delete_password or 
            self.__args.find_password or 
            (self.__args.show and (self.__args.table or self.__args.natural)) 
        )
        isdatabaseenctypted:bool = CheckFile(self.DATABASE_PATH).isdatabaseencryption(self.PASSWORD_FILE_NAME)
        if decrypt and open_data_base:
            try:
                if isdatabaseenctypted:
                    # decrypt database..
                    print('Database is Encryption...')
                    self.set_key()
                    Decryption( paths=[self.PASSWORD_EN_PATH], key=self.__key, remove_files=True ).decrypt()
            except ValueError:
                DisplayError.log("Key Error: Wrong Key!")
                sys.exit(1)

        if not decrypt and not isdatabaseenctypted:
            if not self.__key:
                user_input: str = input('Did you want to encrypt the database [yes/No]: ')
                if user_input.lower()[0] == 'y':
                    user_input = input('Did you have a key [yes/No]: ')
                    if user_input.lower()[0] == 'y':
                        self.set_key()
                else: 
                    return
            try:
                self.__key = Encrytion(paths=[self.PASSWORD_PATH], key=self.__key, remove_files=True).encrypt()
                self.get_key()
            except ValueError:
                DisplayError.log("Key Error: Wrong Key!")
                sys.exit(1)
    # Exit From the xpm
    def exit_handler(self, signum, frame) -> None:
        print('\n------------------ Exit ------------------')
        self.database_stat(False)
        sys.exit(0)
    
    def get_key(self) -> None:
        if not self.__constant_number:
            try:
                self.__constant_number = int(input("Enter the constant number to secure the key (1000 <= x <= 9999): "))
                if x > 9999 or x < 1000: raise Exception
            except:
                DisplayError.log("ErrorInput: The input is not in range (1000 <= x <= 9999), I will generate a random number for you!")
                self.__constant_number = randint(  1000, 9999  )
                print( f"Your constant number is: {self.__constant_number}" )
        secure_ket: bytes = Key.making(
            key= self.__key,
            n= self.__constant_number,
            secure= True
        )
        DisplayKey.log( secure_ket.decode() )

    def set_key(self) -> None:
        try:
            secure_ket: str = input("Enter the secure key: ")
            self.__constant_number = int(input("Enter the constant number: "))
            self.__key = Key.making(
                key= secure_ket.encode(),
                n= self.__constant_number,
                secure= False
            )
        except Exception as error:
            DisplayError.log(error)
            sys.exit(1)

if __name__=="__main__":
    parser:ArgumentParser = ArgumentParser(
		prog="xpm.py",
		epilog="Created by `Mohaned Sherhan (MR.X)`.",
	)

    # Password Management...
    password_management = parser.add_argument_group( 
        'Password Management',
        'A tool used to work with passwords. Set `-pm or --password-management` to use.'
    )
    password_management.add_argument(
        '-pm',
        '--password-management',
        help="Use Password Management tool.",
        action='store_true'
    )
    password_management.add_argument(
        '-l',
        '--length',
        metavar='Number',
        help="Using for setting the length of the password.",
        type=int
    )
    password_management.add_argument(
        '-s',
        '--save',
        help='Save The Password.',
        action='store_true'
    )
    password_management.add_argument(
        '-up',
        '--update-password',
        help="Update the password info.",
        action='store_true'
    )
    password_management.add_argument(
        '-d',
        '--delete-password',
        help="Delete the password.",
        action='store_true'
    )
    password_management.add_argument(
        '-fp',
        '--find-password',
        help="Find password by name.",
        action='store_true'
    )
    password_management.add_argument(
        '-sh',
        '--show',
        help="Show Password using [-t --table] or [-n --natural].",
        action='store_true'
    )
    password_management.add_argument(
        '-t',
        '--table',
        help="Set show as table.",
        action='store_true'
    )
    password_management.add_argument(
        '-n',
        '--natural',
        help="Set show as natural.",
        action='store_true'
    )

    # File Management...
    file_management = parser.add_argument_group(
        'File Management',
        'A tool used to work with files. Set `-fm or --file-management` to use.'
    )
    file_management.add_argument(
        '-fm',
        '--file-management',
        help="Use File Management tool.",
        action='store_true'
    )
    file_management.add_argument(
        '-ef',
        '--encrypt-file',
        help="Encrypt file.",
        action='store_true'
    )
    file_management.add_argument(
        '-df',
        '--decrypt-file',
        help="Decrypt file.",
        action='store_true'
    )
    file_management.add_argument(
        '-efs',
        '--encrypt-folder',
        help="Encrypt folder.",
        action='store_true'
    )
    file_management.add_argument(
        '-dfs',
        '--decrypt-folder',
        help="Decrypt folder.",
        action='store_true'
    )

    # Backup Management...
    backup_management = parser.add_argument_group(
        'Backup Management',
        'A tool use to backup file of folders. Set `-bm or --backup-management` to use.'
    )
    backup_management.add_argument(
        '-bm',
        '--backup-management',
        help="Use Backup Management tool.",
        action="store_true"
    )
    backup_management.add_argument(
        '-pdb',
        '--password-database',
        help="Backup the password Management database.",
        action="store_true"
    )
    backup_management.add_argument(
        '-fb',
        '--folder-backup',
        help="folder backup.",
        action="store_true"
    )
    backup_management.add_argument(
        '-lb',
        '--local-backup',
        help="Local backup with path.",
        action="store_true"
    )
    backup_management.add_argument(
        '-mb',
        '--mega-backup',
        help="Mega.nz backup.",
        action="store_true"
    )
    backup_management.add_argument(
        '-pr',
        '--password-recovery',
        help="Recovery password management backup.",
        action="store_true"
    )
    backup_management.add_argument(
        '-fr',
        '--folder-recovery',
        help="Recovery Folder backup.",
        action="store_true"
    )


    # python xpm.py 
    if len(sys.argv) == 1 :
        parser.print_help(sys.stderr)
        sys.exit(1)

    xpm = Main( parser.parse_args() )
    xpm.run()
