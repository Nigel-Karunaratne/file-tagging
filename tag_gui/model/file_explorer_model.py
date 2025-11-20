from PySide6.QtCore import Qt, QAbstractItemModel, QModelIndex

class FileExplorerModel(QAbstractItemModel):
    def __init__(self, items: list):
        super().__init__()
        self.items: list = items  # List of tuples (name, full_path, custom_data)

    def rowCount(self, parent: QModelIndex): # type: ignore
        if not parent.isValid():
            return len(self.items)
        return 0

    def columnCount(self, parent: QModelIndex): # type: ignore
        return 2  # Two columns: name + tags

    def data(self, index: QModelIndex, role: int): # type: ignore
        if not index.isValid():
            return None
        item = self.items[index.row()]
        if role == Qt.ItemDataRole.DisplayRole:
            if index.column() == 0:
                return item[0]  # File/Folder name
            elif index.column() == 1:
                return item[2]  # Custom data (tag, etc.)
        return None

    def parent(self, index: QModelIndex): # type: ignore
        return QModelIndex()

    def index(self, row: int, column: int, parent: QModelIndex): # type: ignore
        if parent.isValid() or column > 1:
            return QModelIndex()
        return self.createIndex(row, column, self.items[row])
