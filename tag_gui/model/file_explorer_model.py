from typing import Any
from PySide6.QtCore import Qt, QAbstractItemModel, QModelIndex, QPersistentModelIndex
from PySide6.QtWidgets import QApplication, QTreeView, QVBoxLayout, QWidget, QFileSystemModel

import os

class FileExplorerModel(QFileSystemModel):
    def __init__(self, starting_dir):
        super().__init__()
        self.current_directory = starting_dir
        self.current_dir_tags: dict = {}
        self.setRootPath(starting_dir)

    def columnCount(self, parent: QModelIndex | QPersistentModelIndex = QModelIndex()):
        return super().columnCount() + 1

    def data(self, index, role: int = Qt.ItemDataRole.DisplayRole):
        if not index.isValid():
            return None
        if index.column() > 1:
            return None
        
        if index.column() == 1:
            if role == Qt.ItemDataRole.DisplayRole:
                file_name = self.fileName(index.siblingAtColumn(0))
                return self.current_dir_tags.get(file_name)
            elif role == Qt.ItemDataRole.TextAlignmentRole:
                return Qt.AlignmentFlag.AlignLeft | Qt.AlignmentFlag.AlignVCenter
        else:
            return super().data(index, role)
    
    def headerData(self, section: int, orientation: Qt.Orientation, /, role: int = Qt.ItemDataRole.DisplayRole):
        if role == Qt.ItemDataRole.DisplayRole and orientation == Qt.Orientation.Horizontal:
            match section:
                case 0:
                    return "Name"
                case 1:
                    return "Tags"
        return super().headerData(section, orientation, role)

    def set_directory(self, new_directory_path):
        print("CHANIGN PATHS")
        self.current_directory = new_directory_path
        #self.current_dir_tags = {"file1.txt", "None"}
        self.current_dir_tags["file1.txt"] = "No tags a"
        self.setRootPath(new_directory_path)
