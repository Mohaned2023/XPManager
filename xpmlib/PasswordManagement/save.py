from ..ctype.TypeChecker import Check
from json                import loads, dumps 
from ..Error             import FoundError


class SavePassword:
    """Save Password to str"""
    def __init__(self, name: str, password: str, data: bytes) -> None :
        """
        Args:
            name (str): Password name
            password (str): Password
            data (bytes): Database
        """
        Check.check_type(values=[name, data], types=[str, bytes])
        self.__name = name
        self.__password = password
        self.__data = data

    def save(self) -> bytes: 
        """Save the password.

        Raises:
            FoundError: If name is found.

        Returns:
            bytes: Update database.
        """
        passwords: dict = loads(self.__data or '{}')
        if self.__name in passwords.keys(): raise FoundError(f'The name `{self.__name}` found in the passwords data!')
        passwords.update( { self.__name : self.__password } )
        return dumps(passwords).encode()