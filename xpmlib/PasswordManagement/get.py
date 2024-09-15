from ..ctype.TypeChecker import Check
from json                import loads
from ..Error             import NotFoundError, ParameterError

class GetPassword:
    """Get Password from str."""
    def __init__(self,*, data: bytes, name: str=None) -> None:
        """
        Args:
            data (bytes): The password database.
            name (str, optional): Password name - Defaults to None.
        """
        Check.check_type( values=[data], types=[bytes] )
        self.__name = name
        self.__passwords = loads( data or "{}" )
    
    def get_password( self ) -> str:
        """Get One Password

        Raises:
            ParameterError: If there in no name.
            NotFoundError: If the name is not found.

        Returns:
            str: The Password.
        """
        if not self.__name: raise ParameterError("Missing the `name` value!")
        if self.__name not in self.__passwords.keys(): raise NotFoundError(f"The name `{self.__name}` NOT found in the passwords!")
        return self.__passwords[self.__name]

    def get_passwords(self) -> dict:
        """Get All Passwords.

        Returns:
            dict: The Passwords.
        """
        return self.__passwords