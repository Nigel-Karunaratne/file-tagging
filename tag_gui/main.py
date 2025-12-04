import sys
import os
import pathlib

from PySide6.QtWidgets import QApplication
from PySide6.QtGui import QIcon

from view.main_window import MainWindow
from controller.app_controller import AppController
from model.file_query_model import FileQueryModel
from model.tag_model import TagModel
from model.file_explorer_model import FileExplorerModel

def main():
    # Handle command-line args
    app_cwd = os.getcwd()

    for arg in sys.argv:
        if os.path.isdir(arg):
            app_cwd = arg
    # ...for


    """ *********************************** """
    # Create the application
    app = QApplication(sys.argv)
    app.setWindowIcon(QIcon.fromTheme(QIcon.ThemeIcon.FolderNew))

    # Create view, models, and controller
    view = MainWindow()
    view.resize(900, 650)

    tag_model = TagModel(app_cwd)
    query_model = FileQueryModel()
    fs_model = FileExplorerModel(app_cwd)

    script_file_path = pathlib.Path(__file__).parent.resolve()

    controller = AppController(script_file_path, app, fs_model, tag_model, query_model, view)
    
    # Show window
    view.show()
    
    # Event Loop
    sys.exit(app.exec())

if __name__ == "__main__":
    main()
