
from ..Color  import Color
from .display import Display
class DisplayError(Display):
    """Print Errors."""
    @staticmethod
    def log( e:str ) -> None:
        """Print error with red color.

        Args:
            e (str): The error.
        """
        print(f'{Color.URED}{e}{Color.ORG}')