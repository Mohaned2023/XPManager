
from ..ctype.TypeChecker import Check
from ..Color             import Color
from os.path             import basename, isdir, join
from os                  import listdir
from tarfile             import open as taropen

class Tar:
    """Tar Folder."""
    def __init__( self, folder_path: str, save_path: str ) -> None:
        """
        Args:
            folder_path (str): Folder Path.
            save_path (str): Save Path

        Raises:
            NotADirectoryError: If paths are not a directory.
        """
        Check.check_type( values=[folder_path, save_path], types=[str, str] )
        self.__folder_path = folder_path
        self.__save_path = save_path
        if not isdir( self.__folder_path ): raise NotADirectoryError(f"`{self.__folder_path}` is not a directory!")
        if not isdir( self.__save_path ): raise NotADirectoryError(f"`{self.__save_path}` is not a directory!")

    def folder( self ) -> str:
        """Folder Tar

        Returns:
            str: Outfile path.
        """
        folder_name: str = basename( self.__folder_path )
        outfile_path = join(self.__save_path, f"{folder_name}.tar.gz")
        with taropen( outfile_path , 'w:gz' ) as tar :
            tar.add( self.__folder_path, arcname=folder_name )
        return outfile_path