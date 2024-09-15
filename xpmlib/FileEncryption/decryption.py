
from cryptography.fernet import Fernet
from os                  import remove

from ..Display           import DisplayRemoving, DisplaySetting, DisplayEnOrDe
from ..ctype.TypeChecker import Check
from ..File.Read         import FileReader
from ..File.Write        import WriteFile
from ..Color             import Color

class Decryption:
    """Decrypt Files."""
    def __init__( self, paths: list[str], key: bytes, remove_files: bool = False) -> None:
        """
        Args:
            paths (list[str]): Files of folders path.
            key (bytes): Decryption key
            remove_files (bool, optional): Remove files? - Defaults to False.
        """
        Check.check_type(values=[paths], types=[list])
        self.paths = paths
        self.__key = key
        self.__remove_files = remove_files
    
    def decrypt( self ) -> str:
        """Decrypt Files.

        Returns:
            str: Last decrypt file path.
        """
        de_path: str = None
        for path in self.paths:
            if path[-1] != 'x':
                print(f'{Color.GREEN}Not Encryption file `{path}`{Color.ORG}')
                continue

            print('---------- Decryption ----------')
            de_path = path[::-1].replace('x.','', 1)[::-1]
            # Read The file
            file = FileReader(path)
            data_en: bytes = file.read()

            # Encrypt the data
            DisplayEnOrDe.log( path, 'Decrypting')
            cypher = Fernet(self.__key)
            data: bytes = cypher.decrypt(data_en)

            # Write to the file
            file = WriteFile( de_path, data )
            file.write()
            DisplaySetting.log( de_path )

            if self.__remove_files:
                remove(path)
                DisplayRemoving.log( path )
        return de_path