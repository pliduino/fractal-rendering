.PHONY: all run

PYENV := env/Scripts/python.exe
PYSRC := python_rendering


all:
	install.bat

run:
	@$(PYENV) $(PYSRC)/main.py