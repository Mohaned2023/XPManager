from ..ctype.TypeChecker import Check
class Key:
    """Secure the key."""
    @staticmethod
    def making( *, key: bytes, n: int, secure: bool ) -> bytes:
        """Encode the key or decode the key.

        Args:
            key (bytes): The key or encode key.
            n (int): The secure number.
            secure (bool): Encoded the key or decoded.

        Raises:
            ValueError: The range of n not (1000 <= n <= 9999).

        Returns:
            bytes: The encoded key or the decoded key.
        """
        Check.check_type(values=[key, n, secure], types=[bytes, int, bool])
        if n > 9999 or n < 1000 : raise ValueError("The constant must be in range (1000 <= x <= 9999)!")
        mark: str = '%$%'
        new: str = ""
        if secure:
            new = mark.join( hex( ord(i) * n ) for i in key.decode() )
        else:
            for i in key.decode().split(mark):
                new += chr( int(i, 16) // n )
        return new.encode()