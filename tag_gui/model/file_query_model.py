from PySide6.QtCore import Qt, QFileInfo, QDateTime
from PySide6.QtGui import QStandardItemModel, QStandardItem, QIcon
from PySide6.QtWidgets import QFileIconProvider

import os
import sys, subprocess

class FileQueryModel(QStandardItemModel):
    def __init__(self):
        super().__init__()
        self.mapping = dict()
        self._icon_provider = QFileIconProvider()
        self.setHeaderData(0, Qt.Orientation.Horizontal, "Name")
        self.setHeaderData(0, Qt.Orientation.Horizontal, "Tags")

    def headerData(self, section: int, orientation: Qt.Orientation, /, role: int = Qt.ItemDataRole.DisplayRole):
        if role == Qt.ItemDataRole.DisplayRole and orientation == Qt.Orientation.Horizontal:
            match section:
                case 0:
                    return "Name"
                case 1:
                    return "Tags"
        return super().headerData(section, orientation, role)
    
    def rebuild_from_mapping(self):
        self.clear()
        self.beginResetModel()
        print(f"  Tags list is {self.mapping}")
        for path, tags in self.mapping.items():
            print(f"   path is {path}, tags is {tags}")
            info = QFileInfo(path)

            name = QStandardItem(info.fileName())
            name.setEditable(False)
            name.setIcon(self._icon_provider.icon(info))
            name.setData(info.absoluteFilePath(), Qt.ItemDataRole.UserRole)

            # tags_list = []
            # for tag in tags:
            #     if isinstance(tag, list) and len(tag) >= 2:
            #         tags_list.append(f"{tag[0]}: {tag[1]}")
            #     else:
            #         tags_list.append(tag)
            # tags_txt = ",".join(tags_list) if len(tags_list) > 0 else "None"
            # tags_item = QStandardItem(tags_txt)
            tags_item = QStandardItem(tags)
            tags_item.setEditable(False)
            self.appendRow([name, tags_item])
        self.endResetModel()

    def get_file_info_from_index(self, index):
        index = index.siblingAtColumn(0)
        path = index.data(Qt.ItemDataRole.UserRole)
        return QFileInfo(path) if path else None
    
    def get_icon_from_info(self, info) -> QIcon:
        return self._icon_provider.icon(info)

    def open_file_info_from_index(self, index):
        index = index.siblingAtColumn(0)
        path = index.data(Qt.ItemDataRole.UserRole)
        file_info = QFileInfo(path)
        if file_info == None:
            return
        file_path = file_info.absoluteFilePath()
        if os.path.isfile(file_path): # Platform-Specific code for opening a file
            if sys.platform.startswith("win"): # WINDOWS
                os.startfile(file_path)
            elif sys.platform.startswith("darwin"): # MAC
                subprocess.run(["open", file_path])
            else: # ASSUME LINUX
                subprocess.run(["xdg-open", file_path])