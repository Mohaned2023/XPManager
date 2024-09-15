
from .display import Display
from ..Color  import Color
class DisplayRemoving(Display):
    """Display `removing` with style."""
    @staticmethod
    def log( path: str ) -> None:
        print( f'{Color.RED}Removing: {Color.BLUE}{path}{Color.ORG}' )