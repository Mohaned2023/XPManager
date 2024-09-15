
from .display import Display
from ..Color  import Color
class DisplaySetting(Display):
    """Display `setting` with style."""
    @staticmethod
    def log( path:str ) -> None:
        print( f'{Color.RED}Setting: {Color.BLUE}{path}{Color.ORG}' )