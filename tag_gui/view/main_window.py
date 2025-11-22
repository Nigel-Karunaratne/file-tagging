import sys
from PySide6.QtCore import QLocale, QPoint, QRect, QSize, Qt, QDir
from PySide6.QtGui import QCursor, QFont, QIcon, QPalette, QRegion, QAction
from PySide6.QtWidgets import (
    QApplication, QSizePolicy, QWidget, QTabWidget, QVBoxLayout, QLabel, QSplitter, QFileSystemModel, QTreeView, QToolBar, QWidgetAction, QMenuBar, QMenu
)

class MainWindow(QWidget):
    def __init__(self):
        super().__init__()
        self.setWindowTitle("File Tag GUI")

        layout = QVBoxLayout(self)
        
        menu_bar = self._make_menu_bar()
        layout.addWidget(menu_bar)

        self.tabs = QTabWidget()
        files_tab = FilesTab()
        self.tabs.addTab(files_tab, "Files")

        # Tab 2 (tag explorer)
        tag_tab = QWidget()
        tag_tab_layout = QVBoxLayout(tag_tab)
        tag_tab_layout.addWidget(QLabel("Tags")) 
        self.tabs.addTab(tag_tab, "Tag")


        # Add to main layout

        layout.addWidget(self.tabs)

    def _make_menu_bar(self) -> QMenuBar:
        menu_bar = QMenuBar()
        
        file_menu = menu_bar.addMenu("File")
        newAction = QAction(QIcon.fromTheme("document-new"), "&New", self)
        newAction.setShortcut("Ctrl+N")
        newAction.setStatusTip("Create a new file")
        newAction.triggered.connect(self.newFile)
        file_menu.addAction(newAction)


        workspace_menu = menu_bar.addMenu("Workspaces")
        open_or_create_action = QAction(QIcon.fromTheme("document-open"), "&Create/Open Workspace", self)
        open_or_create_action.setShortcut("Ctrl+O")
        open_or_create_action.setStatusTip("Creates or opens a Workspace by name")
        open_or_create_action.triggered.connect(self.newFile)
        workspace_menu.addAction(open_or_create_action)

        
        return menu_bar

    def newFile(self):
        print("CLICKED")
    



class FilesTab(QWidget):
    def __init__(self):
        super().__init__()
        layout = QVBoxLayout(self)
        
        splitter_root = QSplitter(Qt.Orientation.Horizontal)

        left_file_hierarchy = QLabel("left")
        #
        # self.model = QFileSystemModel()
        # self.model.setRootPath(QDir.homePath())  # load the filesystem

        # right_file_hierarchy = QTreeView()
        # right_file_hierarchy.setModel(self.model)
        # right_file_hierarchy.setRootIndex(self.model.index(QDir.homePath()))

        # Optional: hide columns you don't want
        # right_file_hierarchy.setColumnWidth(0, 250)
        # right_file_hierarchy.setHeaderHidden(False)

        #
        right_file_hierarchy = QLabel("right")

        splitter_root.addWidget(left_file_hierarchy)
        splitter_root.addWidget(right_file_hierarchy)

        layout.addWidget(splitter_root)
        splitter_root.setSizes([40,300])
        

