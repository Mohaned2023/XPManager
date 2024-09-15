
from ..ctype.TypeChecker import Check
from os.path             import isfile, join, basename
from tarfile             import open as taropen
from ..Color             import Color

class TarExtract:
    """Extract Tar."""
    def __init__( self, file_path: str, extract_path: str ):
        """
        Args:
            file_path (str): File Path.
            extract_path (str): Save Path.

        Raises:
            FileNotFoundError: If file not found.
        """
        Check.check_type( values=[file_path, extract_path], types=[str,str] )
        if not isfile( file_path ): raise FileNotFoundError(f'`{file_path}` does not exist!')
        self.__file_path = file_path
        self.__extract_path = extract_path

    def extract(self) -> None:
        """Extract Tar."""
        print(   f'{Color.BLUE}Extract: {Color.GREEN}{self.__file_path}{Color.ORG}'  )
        with taropen(  self.__file_path, 'r:gz'  ) as tar:
            tar.extractall( path= self.__extract_path )
        print(   f'{Color.BLUE}Saved: {Color.GREEN}{self.__extract_path}{Color.ORG}'  )