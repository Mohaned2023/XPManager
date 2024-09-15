
from ..ctype.TypeChecker import Check
from rich.console        import Console
from rich.table          import Table
from ..Error             import EmptyDataError
from ..Color             import Color
from .display            import Display
class DisplayPassword(Display):
    """Display the password"""
    def __init__(self, passwords: dict) -> None:
        Check.check_type( values=[passwords], types=[dict] )
        self.__passwords: dict = passwords
        if self.__passwords == {}: raise EmptyDataError("The database is empty!")
    def table(self) -> None:
        """Display all password in the database as table."""
        table = Table(
            title="Passwords",
            caption="Note: Passwords longer than 30 characters has been truncated."
        )
        table.add_column("N")
        table.add_column("Name")
        table.add_column("Password",max_width=30)
        print("\n")
        i:int = 1
        # Create the rows.. 
        for key in self.__passwords.keys() :
            table.add_row( str(i), key, self.__passwords[key], style='bright_green')
            i += 1 
        # Print the Table.. 
        Console().print(table)
        print("\n")

    def natural(self) -> None:
        """Display all password in the database"""
        i: int = 1 
        for key in self.__passwords.keys():
            print(f"{Color.BLUE}{i}- {key}: {Color.GREEN}{self.__passwords[key]}{Color.ORG}")
            i += 1

    @staticmethod
    def log( password: str ) -> None:
        """Log the password to the console.

        Args:
            password (str): The Password.
        """
        print( f'{Color.BLUE}Password: {Color.GREEN}{password}{Color.ORG}' )