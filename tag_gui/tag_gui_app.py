from PySide6.QtWidgets import QApplication, QMainWindow, QTreeView, QVBoxLayout, QWidget, QLineEdit, QFileSystemModel
from PySide6.QtCore import QDir

import os
from model.application_model import ApplicationModel
# import tags

class TagGUIApplication(QApplication):
    def __init__(self, argv: list):
        super().__init__(argv)