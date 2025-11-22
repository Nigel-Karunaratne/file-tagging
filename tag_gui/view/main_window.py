import sys
from PySide6.QtCore import Qt, QDir, QModelIndex
from PySide6.QtGui import QIcon, QAction
from PySide6.QtWidgets import (
    QWidget, QTabWidget, QVBoxLayout, QHBoxLayout, QLabel, QSplitter, QFileSystemModel, QTreeView, QMenuBar, QHeaderView, QPushButton
)
from model.file_explorer_model import FileExplorerModel

import os

class MainWindow(QWidget):
    def __init__(self):
        super().__init__()
        self.setWindowTitle("File Tag GUI")

        self.fs_model = FileExplorerModel(os.getcwd())

        layout = QVBoxLayout(self)
        
        menu_bar = self._make_menu_bar()
        layout.addWidget(menu_bar)

        self.tabs = QTabWidget()
        files_tab = FilesTab(self.fs_model)
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
    def __init__(self, model: FileExplorerModel):
        super().__init__()
        layout = QVBoxLayout(self)

        nav_bar = QHBoxLayout()
        up_btn = QPushButton("Up")
        up_btn.setIcon(QIcon.fromTheme(QIcon.ThemeIcon.GoUp))
        up_btn.clicked.connect(self._on_directory_up_btn_clicked)
        nav_bar.addWidget(up_btn)
        layout.addLayout(nav_bar)
        
        splitter_root = QSplitter(Qt.Orientation.Horizontal)

        self.left_file_hierarchy = QLabel("left")
        
        self.model = model
        self.model.setRootPath(QDir.homePath())  # load the filesystem

        self.right_file_hierarchy = QTreeView()
        self.right_file_hierarchy.setModel(self.model)
        self.right_file_hierarchy.setRootIndex(self.model.index(self.model.current_directory))
        for col in range(2, self.model.columnCount()):
            self.right_file_hierarchy.setColumnHidden(col, True)
        self.right_file_hierarchy.setItemsExpandable(False)
        self.right_file_hierarchy.setRootIsDecorated(False)
        header = self.right_file_hierarchy.header()
        header.setSectionResizeMode(QHeaderView.ResizeMode.Interactive)
        self.right_file_hierarchy.doubleClicked.connect(self._on_file_folder_double_click)


        """"""

        # right_file_hierarchy.setColumnWidth(0, 250)
        # right_file_hierarchy.setHeaderHidden(False)
        
        # right_file_hierarchy = QLabel("right")

        splitter_root.addWidget(self.left_file_hierarchy)
        splitter_root.addWidget(self.right_file_hierarchy)

        layout.addWidget(splitter_root)
        splitter_root.setSizes([40,300])

    def _on_file_folder_double_click(self, index: QModelIndex):
        name = index.siblingAtColumn(0)
        print(f"NAME: {name}")
        if self.model.isDir(name):
            file_name = self.model.fileName(name)
            file_path = self.model.filePath(name)
            self.model.set_directory(file_path)
            self.right_file_hierarchy.setRootIndex(self.model.index(file_path))
        return
    
    def _on_directory_up_btn_clicked(self):
        current_dir = self.model.current_directory
        parent_dir = os.path.dirname(current_dir)
        if parent_dir and os.path.exists(parent_dir):
            self.model.set_directory(parent_dir)
            self.right_file_hierarchy.setRootIndex(self.model.index(parent_dir))
        

