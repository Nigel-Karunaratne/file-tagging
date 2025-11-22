import sys
import os

from model.application_model import ApplicationModel

from PySide6.QtWidgets import QApplication
from view.main_window import MainWindow

def main():
    # Handle command-line args
    app_cwd = "."

    for arg in sys.argv:
        if os.path.isdir(arg):
            app_cwd = arg
    # ...for


    """ *********************************** """
    # Create the application
    app = QApplication(sys.argv)
    
    # Create the root widget / view
    view = MainWindow()
    view.resize(800, 650)

    # TODO - create controller? Or have View just own the model?
    # Use 1 model (files + tags)? Or have 2 models, where QFileSystemModel references other?
    
    # Show window
    view.show()
    
    # Event Loop
    sys.exit(app.exec())

if __name__ == "__main__":
    main()
