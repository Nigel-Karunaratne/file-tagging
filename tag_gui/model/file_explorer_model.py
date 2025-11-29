from PySide6.QtCore import Qt, QAbstractItemModel, QModelIndex, QPersistentModelIndex, Signal
from PySide6.QtWidgets import QApplication, QTreeView, QVBoxLayout, QWidget, QFileSystemModel

import os, sys, subprocess

class FileExplorerModel(QFileSystemModel):
    sg_going_up_directory = Signal(str)

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

    def set_directory(self, new_directory_path, mapping: dict):
        print(f"CHANIGN PATHS (mapping IS {mapping})")
        self.current_directory = new_directory_path
        self.current_dir_tags = mapping
        self.setRootPath(new_directory_path)
    
    # Returns (FALSE, directory_path) if DIRECTORY, TRUE,"" if FILE
    def open_file_info_from_index(self, index) -> tuple[bool, str]:
        name = index.siblingAtColumn(0)
        file_path = self.filePath(name)
        if self.isDir(name):
            file_name = self.fileName(name)
            # self.set_directory(file_path, self.current_directory)
            return (False, file_path)
        elif self.fileInfo(name).isFile():
            # Platform-Specific code for opening a file
            if sys.platform.startswith("win"): # WINDOWS
                os.startfile(file_path)
            elif sys.platform.startswith("darwin"): # MAC
                subprocess.run(["open", file_path])
            else: # ASSUME LINUX
                subprocess.run(["xdg-open", file_path])
        return (True, "")
    
    def attempt_to_go_up(self):
        parent_dir = os.path.dirname(self.current_directory)
        if parent_dir and os.path.exists(parent_dir):
            # self.current_directory = parent_dir
            self.sg_going_up_directory.emit(parent_dir)
        return
