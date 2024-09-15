from os.path import isfile
from ..ctype.TypeChecker import Check


class FileReader:
    """Read files."""
    def __init__(self, path: str) -> None:
        """
        Args:
            path (str): File path.
        """
        Check.check_type( values=[path], types=[str] )
        self.path = path

    def read( self ) -> bytes:
        """Read the file.

        Raises:
            FileNotFoundError: if file not exist.

        Returns:
            bytes: File data as bytes.
        """
        if not isfile( self.path ) :
            raise FileNotFoundError(f"{self.path} does not exist!")
        
        with open( self.path, 'rb' ) as file:
            return file.read()