
from cryptography.fernet import Fernet
from os                  import remove

from ..Display           import DisplaySetting, DisplayRemoving, DisplayKey, DisplayEnOrDe
from ..File.Read         import FileReader
from ..File.Write        import WriteFile
from ..ctype.TypeChecker import Check
from ..Color             import Color

class Encrytion:
    """Encrypt Files"""
    def __init__( self, paths: list[str], key: bytes=None, remove_files: bool = False ) -> None:
        """
        Args:
            paths (list[str]): Files or folders paths.
            key (bytes, optional): Encryption key - Defaults to None.
            remove_files (bool, optional): Remove Files? - Defaults to False.
        """
        Check.check_type(values=[paths], types=[list])
        self.paths = paths
        self.__key = key if key else Fernet.generate_key()
        self.__remove_files = remove_files

    def encrypt( self ) -> None:
        """Encrypt files."""
        for path in self.paths:
            print('---------- Encryption ----------')
            en_path: str = path + '.x'

            # Read The file
            file = FileReader(path)
            data: bytes = file.read()

            DisplayEnOrDe.log(path, 'Encrypting')
            # Encrypt the data
            cypher = Fernet(self.__key)
            data_en: bytes = cypher.encrypt(data)

            # Write to the file
            file = WriteFile( en_path, data_en )
            file.write()
            DisplaySetting.log(en_path)

            if self.__remove_files:
                remove(path)
                DisplayRemoving.log(path)
    
        # print the key
        DisplayKey.log(self.__key.decode())
        return