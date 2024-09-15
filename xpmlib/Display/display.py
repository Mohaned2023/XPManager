
from abc import ABC, abstractmethod
class Display(ABC):
    
    @staticmethod
    @abstractmethod
    def log( msg:str ) -> None:...