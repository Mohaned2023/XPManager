from ..ctype.TypeChecker import Check
from os.path             import exists, isdir, join
from ..File.Paths        import GetPath

class CheckFile:
    """Check password management database."""
    def __init__(self, path:str ) -> None :
        """
        Args:
            path (str): File path.
        """
        Check.check_type( values=[path], types=[str] )
        self.__path = path
    
    def isencryption(self) -> bool:
        """Check if the file is encryption.

        Raises:
            FileNotFoundError: If file not exist.

        Returns:
            bool: File Encryption?
        """
        if not exists(self.__path):
            raise FileNotFoundError(f"{self.__path} does not exist!")
        if self.__path[-1] == 'x':
            return True
        return False

    def isdatabaseencryption(self, file_name: str) -> bool:
        """Check the database if is encryption.

        Args:
            file_name (str): Database file name.

        Raises:
            NotADirectoryError: If Folder path not a directory.

        Returns:
            bool: Database is encryption?
        """
        if not isdir(self.__path): raise NotADirectoryError(f'`{self.__path}` is not a directory!')
        if join( self.__path, file_name+'.x') in GetPath(self.__path).get_all_files_paths():
            return True
        return False
