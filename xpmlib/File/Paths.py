from ..ctype.TypeChecker import Check
from os.path import isdir
from pathlib import Path

class GetPath:
    """Find all paths in the folder."""
    def __init__(self, folder_path: str):
        """
        Args:
            folder_path (str): Folder Path.

        Raises:
            NotADirectoryError: If the folder_path is not exist.
        """
        Check.check_type( values=[folder_path], types=[str] )
        if not isdir(folder_path): raise NotADirectoryError(f"`{folder_path}` is not a directory!")
        self.path = folder_path
        self.__all_paths = []

    def get_all_files_paths(self) -> list[str]:
        """Fild all paths in the folder.

        Returns:
            list[str]: List of paths.
        """
        folder_path = Path(self.path)
        for path in folder_path.rglob("*"):
            if path.is_file() :
                self.__all_paths.append( str(path) )
        return self.__all_paths
