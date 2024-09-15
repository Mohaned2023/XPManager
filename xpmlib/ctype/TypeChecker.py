class Check:
    """Check the values type."""
    @staticmethod
    def check_type( *, values: list, types: list[type] )-> bool:
        """Check the type of the values.

        Args:
            values (list): The Values.
            types (list[type]): The Types.

        Raises:
            TypeError: If type of value does not match the type.

        Returns:
            bool
        """
        for value, t in zip(values, types):
            if not isinstance(value, t) :
                raise TypeError(f'Expect `{t.__name__}` found {type(value).__name__} !')