import sys
from PySide6.QtCore import Qt, QDir, QModelIndex
from PySide6.QtGui import QIcon, QAction
from PySide6.QtWidgets import (
    QWidget, QTabWidget, QVBoxLayout, QHBoxLayout, QLabel, QSplitter, QFileSystemModel, QTreeView, QMenuBar, QHeaderView, QPushButton, QStackedWidget, QTextEdit
)
from model.file_explorer_model import FileExplorerModel
from model.tag_model import TagModel

import os
import sys, subprocess # For opening files w/ OS's default program

class MainWindow(QWidget):
    def __init__(self):
        super().__init__()
        self.setWindowTitle("File Tag GUI")

        self.fs_model = FileExplorerModel(os.getcwd())
        self.tag_model = TagModel(os.getcwd())
        self.fs_model.setRootPath(os.getcwd())  # load the filesystem
        self.fs_model.set_directory(os.getcwd(), self.tag_model)

        layout = QVBoxLayout(self)
        
        menu_bar = self._make_menu_bar()
        layout.addWidget(menu_bar)

        self.tabs = QTabWidget()
        files_tab = FilesTab(self.fs_model, self.tag_model)
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
    def __init__(self, fs_model: FileExplorerModel, tag_model: TagModel):
        super().__init__()
        layout = QVBoxLayout(self)

        # ** NAV BAR ** #
        nav_bar = QHBoxLayout()
        up_btn = QPushButton("Up")
        up_btn.setIcon(QIcon.fromTheme(QIcon.ThemeIcon.GoUp))
        up_btn.clicked.connect(self._on_directory_up_btn_clicked)
        nav_bar.addWidget(up_btn)
        layout.addLayout(nav_bar)
        
        self.fs_model = fs_model
        self.tag_model = tag_model

        # ** LEFT SIDE ** #
        self.left_stack = QStackedWidget()
        self.left_file_info_widget = FileInfoWidget()
        self.left_placeholder_widget = QLabel("Select a file to see info...")
        self.left_stack.addWidget(self.left_placeholder_widget) # Index 0
        self.left_stack.addWidget(self.left_file_info_widget) # Index 1
        self.left_stack.setCurrentIndex(0)

        # ** RIGHT SIDE ** #
        self.right_file_hierarchy = QTreeView()
        self.right_file_hierarchy.setModel(self.fs_model)
        self.right_file_hierarchy.setRootIndex(self.fs_model.index(self.fs_model.current_directory))
        for col in range(2, self.fs_model.columnCount()):
            self.right_file_hierarchy.setColumnHidden(col, True)
        self.right_file_hierarchy.setItemsExpandable(False)
        self.right_file_hierarchy.setRootIsDecorated(False)
        header = self.right_file_hierarchy.header()
        header.setSectionResizeMode(0,QHeaderView.ResizeMode.Interactive)
        header.setSectionResizeMode(1,QHeaderView.ResizeMode.ResizeToContents)
        header.setMinimumSectionSize(50)
        self.right_file_hierarchy.setColumnWidth(0, 250)
        self.right_file_hierarchy.doubleClicked.connect(self._on_file_folder_double_click)
        self.right_file_hierarchy.selectionModel().selectionChanged.connect(self._on_file_folder_selection_changed)

        # self.right_file_hierarchy.setHorizontalScrollBarPolicy(Qt.ScrollBarPolicy.ScrollBarAlwaysOff)

        # ** ROOT SPLITTER ** #
        splitter_root = QSplitter(Qt.Orientation.Horizontal)

        splitter_root.addWidget(self.left_stack)
        splitter_root.addWidget(self.right_file_hierarchy)

        layout.addWidget(splitter_root)
        splitter_root.setSizes([40,300])

    def _on_file_folder_selection_changed(self, selected, _deselected):
        indexes = selected.indexes()
        if not indexes or not indexes[0].isValid():
            self.left_stack.setCurrentIndex(0)
        else:
            self.left_stack.setCurrentIndex(1)
            self.left_file_info_widget.set_selected(self.fs_model, indexes[0].siblingAtColumn(0), self.tag_model)
        return

    def _on_file_folder_double_click(self, index: QModelIndex):
        name = index.siblingAtColumn(0)
        file_path = self.fs_model.filePath(name)
        if self.fs_model.isDir(name):
            file_name = self.fs_model.fileName(name)
            self.fs_model.set_directory(file_path, self.tag_model)
            self.right_file_hierarchy.setRootIndex(self.fs_model.index(file_path))
        elif self.fs_model.fileInfo(name).isFile():
            # Platform-Specific code for opening a file
            if sys.platform.startswith("win"): # WINDOWS
                os.startfile(file_path)
            elif sys.platform.startswith("darwin"): # MAC
                subprocess.run(["open", file_path])
            else: # ASSUME LINUX
                subprocess.run(["xdg-open", file_path])
        return
    
    def _on_directory_up_btn_clicked(self):
        current_dir = self.fs_model.current_directory
        parent_dir = os.path.dirname(current_dir)
        if parent_dir and os.path.exists(parent_dir):
            self.fs_model.set_directory(parent_dir, self.tag_model)
            self.right_file_hierarchy.setRootIndex(self.fs_model.index(parent_dir))
        

class FileInfoWidget(QWidget):
    def __init__(self):
        super().__init__()
        layout = QVBoxLayout()
        self.tags = []

        self.label_icon = QLabel()
        self.label_name = QLabel()
        self.label_path = QLabel()
        self.label_size = QLabel()
        self.label_last_modified = QLabel()

        hbox = QHBoxLayout()
        self.add_tag_text_edit = QTextEdit()
        self.add_tag_text_edit.setPlaceholderText("tag...")
        self.add_tag_button = QPushButton("Add Tag")
        self.add_tag_button.setIcon(QIcon.fromTheme(QIcon.ThemeIcon.ListAdd))
        hbox.addWidget(self.add_tag_text_edit)
        hbox.addWidget(self.add_tag_button)

        self.vbox_tags = QVBoxLayout()
        self.label_tags = QLabel("Tags:")
        self.vbox_tags.addWidget(self.label_tags)

        self.label_icon.setWordWrap(True)
        self.label_name.setWordWrap(True)
        self.label_path.setWordWrap(True)
        self.label_size.setWordWrap(True)
        self.label_last_modified.setWordWrap(True)

        layout.addWidget(self.label_icon)
        layout.addWidget(self.label_name)
        layout.addWidget(self.label_path)
        layout.addWidget(self.label_size)
        layout.addWidget(self.label_last_modified)
        layout.addLayout(hbox, stretch=0)
        layout.addLayout(self.vbox_tags)
        self.rebuild_tag_list()

        self.setLayout(layout)
        return
    
    def set_selected(self, fs_model: FileExplorerModel, index: QModelIndex, tag_model: TagModel):
        if not index.isValid():
            self.hide()
            return
        self.label_icon.setPixmap(fs_model.fileIcon(index).pixmap(32,32))
        self.label_name.setText(f"Name: {fs_model.fileName(index)}")
        self.label_path.setText(f"Path: {fs_model.filePath(index)}")
        self.label_size.setText(f"Size: {fs_model.size(index)}")
        lm = fs_model.lastModified(index)
        self.label_last_modified.setText(f"Last Modified: {fs_model.lastModified(index).toString()}")

        # self.tags = tag_model.get_tags_for_filename(fs_model.filePath(index));
        self.tags = tag_model.get_tags_for_filename_as_list_of_str(fs_model.filePath(index))
        self.rebuild_tag_list()

        self.show()
        return
    
    def rebuild_tag_list(self):
        # Remove Rows
        while self.vbox_tags.count():
            child = self.vbox_tags.takeAt(0)
            if child.widget():
                child.widget().deleteLater() # pyright: ignore[reportOptionalMemberAccess]
        # Add Rows
        if len(self.tags) <= 0:
            self.vbox_tags.addWidget(QLabel("No Tags"))
        else:
            self.vbox_tags.addWidget(QLabel("Tags:"))
            for tag in self.tags:
                row = QWidget()
                layout = QHBoxLayout(row)
                label = QLabel(tag)
                delete_btn = QPushButton()
                delete_btn.setIcon(QIcon.fromTheme(QIcon.ThemeIcon.ListRemove))
                delete_btn.clicked.connect(lambda _, value=tag: self.remove_tag_from_active_file(tag))
                layout.addWidget(label)
                layout.addStretch()
                layout.addWidget(delete_btn)
                self.vbox_tags.addWidget(row)
    
    def remove_tag_from_active_file(self, tag):
        print("REMOVING TAG FORM ACTIVE FILE!!!")
        return
