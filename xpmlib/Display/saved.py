
from .display import Display
from ..Color  import Color
class DisplaySaved(Display):
    """Display saved with style."""
    @staticmethod
    def log( outfile_path: str ) -> None:
        """Logging the out file path to the console.

        Args:
            outfile_path (str): The out file path.
        """
        print( f"{Color.BLUE}Saved: {Color.GREEN}{outfile_path}{Color.ORG}" )