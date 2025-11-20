import sys
import os

from view.file_explorer_widget import FileExplorerWidget
from model.application_model import ApplicationModel

from PySide6.QtWidgets import QApplication

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
    
    # Create the app model
    app_model = ApplicationModel(app_cwd)

    # Create the root widget / view
    view = FileExplorerWidget()

    # TODO - create controller
    
    # Show window
    view.show()
    
    # Event Loop
    sys.exit(app.exec())

if __name__ == "__main__":
    main()
