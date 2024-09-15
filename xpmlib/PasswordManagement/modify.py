from ..ctype.TypeChecker    import Check
from json                   import loads ,dumps
from ..Error                import NotFoundError, ParameterError, FoundError
from ..Color                import Color

class ModifyPassword:
    """Modify Password."""
    def __init__(self, name: str, data: bytes) -> None:
        """
        Args:
            name (str): Password name.
            data (bytes): Database.
        """
        Check.check_type( values=[name, data], types=[ str, bytes ])
        self.__name = name
        self.__data = data

    def update( self, *,choice:int=1, new_name:str=None, new_password:str=None) -> bytes:
        """Update the password.

        choice = 1 -> Name.
        choice = 2 -> Password.
        choice = 3 -> Both.

        Args:
            choice (int, optional): The choice - Defaults to 1.
            new_name (str, optional): new name - Defaults to None.
            new_password (str, optional): new password - Defaults to None.

        Raises:
            ParameterError: If ther is no name.
            NotFoundError: If the name is not found.
            FoundError: If the name is found.
            ValueError: If not choice.

        Returns:
            bytes: update database.
        """
        Check.check_type(values=[choice], types=[int])
        passwords: dict = loads(self.__data or '{}')
        # Name
        if choice == 1 :
            if not new_name: raise ParameterError("Missing the `new_name` value!")
            if self.__name not in passwords.keys(): raise NotFoundError(f"The name `{self.__name}` NOT found in the passwords!")
            if new_name in passwords.keys(): raise FoundError(f'The name `{new_name}` found in the passwords data!')
            hold_password = passwords[self.__name]
            del passwords[self.__name]
            passwords.update(  {new_name: hold_password}  )
            print( f'{Color.BLUE}new name: {Color.GREEN}{new_name}{Color.ORG}' )
        # Password
        elif choice == 2:
            if not new_password: raise ParameterError("Missing the `new_password` value!")
            if self.__name not in passwords.keys(): raise NotFoundError(f"The name `{self.__name}` NOT found in the passwords!")
            passwords.update( { self.__name: new_password } )
            print( f'{Color.BLUE}new password: {Color.GREEN}{new_password}{Color.ORG}' )
        # Both
        elif choice == 3:
            self.__data = self.update( choice=1, new_name=new_name)
            self.__name = new_name
            self.__data = self.update( choice=2, new_password=new_password)
            return self.__data
        # Error Input
        else:
            raise ValueError("Input is not in (1,2,3)!")
        return dumps(passwords).encode()

    def delete(self) -> bytes:
        """Delete Password.

        Raises:
            NotFoundError: If the name is not found.

        Returns:
            bytes: update database.
        """
        passwords: dict = loads(self.__data or '{}')
        if self.__name not in passwords.keys(): raise NotFoundError(f"The name `{self.__name}` NOT found in the passwords!")
        del passwords[self.__name]
        return dumps(passwords).encode()