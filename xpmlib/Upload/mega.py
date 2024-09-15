
from ..ctype.TypeChecker import Check
from os.path             import isfile, getsize
from mega                import Mega
from ..Error             import FileSizeError
from ..Color             import Color

class MegaUploader:
    """Upload to mega.nz"""
    def __init__( self, file_path: str, email: str, password: str ) -> None:
        """
        Args:
            file_path (str): File Path
            email (str): Mega Email.
            password (str): Mega Password.

        Raises:
            FileNotFoundError: If is not file.
        """
        Check.check_type( values=[file_path, email, password], types=[str, str, str] )
        if not isfile( file_path ): raise FileNotFoundError(f'`{file_path}` does not exist!')
        self.__file_path = file_path
        self.__user = Mega().login( email= email, password= password ) # requests.exceptions.ConnectionError or mega.errors.RequestError
        self.__cloud_size_giga = self.__user.get_storage_space(giga=True)

    def upload_file(self) -> None:
        """Upload the File.

        Raises:
            FileSizeError: If the cloud size is not enough for the file.
        """
        cloud_free_size_giga = float(self.__cloud_size_giga['total']) - float( self.__cloud_size_giga['used'] )
        file_size_giga = (getsize(self.__file_path) / 1024**3)/8
        if file_size_giga >= cloud_free_size_giga: raise FileSizeError("You don't have enough space on mega!")
        print( f'{Color.BLUE}Uploading: {Color.GREEN}{self.__file_path}{Color.ORG}' )
        self.__user.upload(self.__file_path)
