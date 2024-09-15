
from .display import Display
from ..Color  import Color
class DisplayEnOrDe(Display):
    """Display `encrypting` or `decrypting` with style."""
    @staticmethod
    def log( path: str , enorde: str ) -> None:
        """Logging the encryption or decrypting path to the console.

        Args:
            path   (str): The path.
            enorde (str): The encrypting or decrypting message.
        """
        print( f'{Color.RED}{enorde}: {Color.BLUE}{path}{Color.ORG}' )