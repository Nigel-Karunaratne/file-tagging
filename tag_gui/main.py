import sys
import os

from PySide6.QtWidgets import QApplication
from PySide6.QtGui import QIcon

from view.main_window import MainWindow
from controller.app_controller import AppController

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
    app.setWindowIcon(QIcon.fromTheme(QIcon.ThemeIcon.FolderNew))

    # Create view
    view = MainWindow()
    view.resize(800, 650)

    # TODO - create controller? Or have View just own the model?
    controller = AppController(view.fs_model, view.tag_model, view.files_tab.right_file_info_widget, view.files_tab)
    
    # Show window
    view.show()
    
    # Event Loop
    sys.exit(app.exec())

if __name__ == "__main__":
    main()
