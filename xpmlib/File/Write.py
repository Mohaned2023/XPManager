from ..ctype.TypeChecker import Check

class WriteFile:
    """Write Files."""
    def __init__(self, path: str, data: bytes) -> None:
        """
        Args:
            path (str): File path.
            data (bytes): File data.
        """
        Check.check_type( values=[path, data], types=[str,bytes] )
        self.path = path
        self.data = data

    def write( self ) -> None: 
        """Write to the file."""
        with open( self.path, 'wb' ) as file:
            file.write(self.data)