# PasswordManegement:
A set of clesses that use to generate, modify, save and get passwords.

# Usage:
- GeneratePassword:
```python
from xpmlib.PasswordManagement import GeneratePassword
from xpmlib.Error import LengthError

password_length: int = 128 # ( 8 <= x <= 512 )
try: 
    pass_object: GeneratePassword = GeneratePassword( length= password_length)
    password: str = pass_object.generate()
    print( f"{password = }" )
except LengthError:
    print('Length most be in range of ( 8 <= x <= 512 )!')
```

- GetPassword:
```python
from xpmlib.PasswordManagement import GetPassword
from Error import NotFoundError, ParameterError

data: bytes = b'{"name one": "password one", "name two": "password two"}'
name: str = 'name one'

try:
    get_object: GetPassword = GetPassword( data= data, name= name)
    password: str = get_object.get_password() # Requier a name ^
    print( f"{password = }" )
    all_passwords: dict = get_object..get_passwords()
    print( f"{ all_passwords = }" )
except TypeError:
    print( "One of GetPassword Parameter type is not ture!" )
except ParameterError:
    print("Missing the `name` value when you used get_password function!")
except NotFoundError:
    print(f"The name `{name}` NOT found in the passwords!")
```

- ModifyPassword:
```python
from xpmlib.PasswordManagement import ModifyPassword
from xpmlib.Error import ParameterError, NotFoundError, FoundError

data: bytes = b'{"name one": "password one", "name two": "password two"}'
name: str = 'name one'

try: 
    modify: ModifyPassword = ModifyPassword( name= name, data= data )
    # Update ...
    try: 
        new_name: str = 'new name'
        new_data: bytes = modify.update( choice= 1, new_name= new_name) # set choice to 1 for name
        new_password: str = "new password"
        new_data: bytes = modify.update( choice= 2, new_password= new_password ) # set choice to 2 for password
        new_name = "both - new name"
        new_password = "both - new password"
        new_data: bytes = modify.update( choice= 3, new_name= new_name, new_password= new_password ) # set choice to 3 for name and password
    except ParameterError:
        print("Missing the `new_name` value!")
    except NotFoundError:
        print(f"The name `{name}` NOT found in the passwords!")
    except FoundError:
        print(f'The name `new_name value` found in the passwords data!')
    except ValueError:
        print("Choice is not in (1,2,3)!")
    # Delete ...
    try: 
        new_data: bytes = modify.delete()
        print( f"{new_data = }" )
    except NotFoundError:
        print(f"The name `{name}` NOT found in the passwords!")
except TypeError: 
    print( "One of ModifyPassword Parameter type is not ture!" )
```

- SavePassword:
```python
from xpmlib.PasswordManagement import SavePassword
from xpmlib.Error import NotFoundError

data: bytes = b'{"name one": "password one", "name two": "password two"}'
name: str = 'new one'
password: str = "new password"
try: 
    save_object: SavePassword = SavePassword( name= name, password= password, data= data )
    new_data: bytes = save_object.save()
    print( f"{new_data = }" )
except TypeError: 
    print( "One of SavePassword Parameter type is not ture!" )
except NotFoundError:
    print(f"The name `{name}` NOT found in the passwords!")
```
