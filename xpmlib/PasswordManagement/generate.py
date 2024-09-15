from ..ctype.TypeChecker import Check
from ..Error             import LengthError
from string              import ascii_letters, digits, punctuation
from random              import choice

class GeneratePassword:
    """Generate Password."""
    def __init__(self, length: int ) -> None:
        """
        Args:
            length (int): The length of the password.

        Raises:
            LengthError: If length < 8 or length > 512.
        """
        Check.check_type( values=[length], types=[int] )
        if not (length >= 8 and length <= 512):
            raise LengthError('Length most be in range of ( 8 <= x <= 512 )!')
        self.__length = length
        self.__ascii = ascii_letters+digits+punctuation.replace('\\', '')
    
    def generate(self) -> str:
        """Generate the password.

        Returns:
            str: The password.
        """
        return ''.join( choice(self.__ascii) for _ in range(self.__length) )