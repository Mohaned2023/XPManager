
from .display import Display
from ..Color  import Color
class DisplayKey(Display):
    """Display key with style."""
    @staticmethod
    def log( key: str ) -> None:
        """Logging the key to the console.

        Args:
            key (str): The key.
        """
        print( f"{Color.BLUE}Key: {Color.GREEN}{key}{Color.ORG}" )