# Windows:

``` make
make 
```

``` make
make run
```

# Linux:

Create a virtual environment and install the requirements. Next build the rust code and then run it:

``` bash
    python3 -m venv env
    ./env/bin/activate

    pip install -Ur requirements.txt

    cd rust_generation
    maturin develop
    cd ..

    ./python_rendering/main.py
```
