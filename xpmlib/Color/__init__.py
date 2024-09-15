class Color:
    '''
    A Class to get colors.

    This class has some colors to coordinate.

    Example::
        # To use this class
        print( f"{Color.RED}Hello python{Color.ORG}" )
    '''
    RED: str = '\033[0;31m'
    URED: str = '\033[4;31m'
    GREEN: str = '\033[0;32m'
    BLUE: str = '\033[0;34m'
    ORG: str = '\033[0m'
