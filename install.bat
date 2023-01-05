@set PY=python
@set VENV=env

@%PY% -m venv %VENV%

@call .\%VENV%\Scripts\activate

@%PY% -m pip install -Ur requirements.txt

@cd .\rust_generation
@maturin develop
@cd ..