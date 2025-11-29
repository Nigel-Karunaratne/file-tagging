import sys
from PySide6.QtCore import Qt, QDir, QModelIndex, Signal
from PySide6.QtGui import QIcon, QAction
from PySide6.QtWidgets import (
    QWidget, QTabWidget, QVBoxLayout, QHBoxLayout, QLabel, QSplitter, QFileSystemModel, QTreeView, QMenuBar, QHeaderView, QPushButton, QStackedWidget, QLineEdit, QScrollArea, QSizePolicy, QCheckBox, QRadioButton, QGroupBox
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

        self.files_tab = FilesTab(self.fs_model, self.tag_model)
        self.tabs.addTab(self.files_tab, "Files")

        # Tab 2 (tag explorer)
        query_tab = QueryTab(self.fs_model, self.tag_model) 
        self.tabs.addTab(query_tab, "Search")

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
        self.workspace_name_label = QLabel("...")
        self.workspace_name_label.setSizePolicy(QSizePolicy.Policy.Minimum, QSizePolicy.Policy.Fixed)
        self.workspace_name_label.setAlignment(Qt.AlignmentFlag.AlignHCenter | Qt.AlignmentFlag.AlignVCenter)
        self.set_workspace_name_label(tag_model)
        nav_bar.addWidget(self.workspace_name_label)
        layout.addLayout(nav_bar)
        
        self.fs_model = fs_model
        self.tag_model = tag_model

        # ** RIGHT SIDE SIDE ** #
        self.right_stack = QStackedWidget()
        scroll_area = QScrollArea()
        scroll_area.setWidgetResizable(True)
        scroll_area.setHorizontalScrollBarPolicy(Qt.ScrollBarPolicy.ScrollBarAlwaysOff)
        scroll_area.setVerticalScrollBarPolicy(Qt.ScrollBarPolicy.ScrollBarAsNeeded)
        self.right_file_info_widget = FileInfoWidget()
        self.right_placeholder_widget = QLabel("Select a file...")
        self.right_placeholder_widget.setAlignment(Qt.AlignmentFlag.AlignHCenter | Qt.AlignmentFlag.AlignVCenter)
        self.right_stack.addWidget(self.right_placeholder_widget) # Index 0
        self.right_stack.addWidget(self.right_file_info_widget) # Index 1
        self.right_stack.setSizePolicy(QSizePolicy.Policy.Expanding, QSizePolicy.Policy.MinimumExpanding)
        scroll_area.resizeEvent = self.on_scroll_resize
        scroll_area.setWidget(self.right_stack)
        self.right_stack.setCurrentIndex(0)

        # ** LEFT SIDE ** #
        self.left_file_hierarchy = QTreeView()
        self.left_file_hierarchy.setModel(self.fs_model)
        self.left_file_hierarchy.setRootIndex(self.fs_model.index(self.fs_model.current_directory))
        for col in range(2, self.fs_model.columnCount()):
            self.left_file_hierarchy.setColumnHidden(col, True)
        self.left_file_hierarchy.setItemsExpandable(False)
        self.left_file_hierarchy.setRootIsDecorated(False)
        header = self.left_file_hierarchy.header()
        header.setSectionResizeMode(0,QHeaderView.ResizeMode.Interactive)
        header.setSectionResizeMode(1,QHeaderView.ResizeMode.ResizeToContents)
        header.setMinimumSectionSize(50)
        self.left_file_hierarchy.setColumnWidth(0, 250)
        self.left_file_hierarchy.doubleClicked.connect(self._on_file_folder_double_click)
        self.left_file_hierarchy.selectionModel().selectionChanged.connect(self._on_file_folder_selection_changed)

        # self.left_file_hierarchy.setHorizontalScrollBarPolicy(Qt.ScrollBarPolicy.ScrollBarAlwaysOff)

        # ** ROOT SPLITTER ** #
        splitter_root = QSplitter(Qt.Orientation.Horizontal)

        splitter_root.addWidget(self.left_file_hierarchy)
        splitter_root.addWidget(scroll_area)

        layout.addWidget(splitter_root)
        splitter_root.setSizes([250,40])

    def set_workspace_name_label(self, tag_model: TagModel):
        name = tag_model.get_workspace_name()
        if name == "":
            self.workspace_name_label.setText("No Workspace Opened")
        else:
            self.workspace_name_label.setText(f"Workspace: {name}")

    def on_scroll_resize(self, event):
        self.right_stack.setMinimumWidth(event.size().width())
        event.accept()

    def _on_file_folder_selection_changed(self, selected, _deselected):
        indexes = selected.indexes()
        if not indexes or not indexes[0].isValid():
            self.right_stack.setCurrentIndex(0)
        else:
            self.right_stack.setCurrentIndex(1)
            self.right_file_info_widget.set_selected(self.fs_model, indexes[0].siblingAtColumn(0), self.tag_model)
        return

    def _on_file_folder_double_click(self, index: QModelIndex):
        name = index.siblingAtColumn(0)
        file_path = self.fs_model.filePath(name)
        if self.fs_model.isDir(name):
            file_name = self.fs_model.fileName(name)
            self.fs_model.set_directory(file_path, self.tag_model)
            self.left_file_hierarchy.setRootIndex(self.fs_model.index(file_path))
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
            self.left_file_hierarchy.setRootIndex(self.fs_model.index(parent_dir))
        

class FileInfoWidget(QWidget):
    sg_remove_tab_button_clicked = Signal(str, str, object)
    sg_add_simple_button_clicked = Signal(str, str)
    sg_add_kv_button_clicked = Signal(str, str, str)

    def __init__(self):
        super().__init__()
        layout = QVBoxLayout()
        self.tags = []
        self.current_file_path = ""

        self.label_icon = QLabel()
        self.label_name = QLabel()
        self.label_path = QLabel()
        self.label_size = QLabel()
        self.label_last_modified = QLabel()

        # hbox_simple = QHBoxLayout()
        self.add_simple_tag_text_edit = QLineEdit()
        self.add_simple_tag_text_edit.setPlaceholderText("tag...")
        self.add_simple_tag_button = QPushButton("Add Simple Tag")
        self.add_simple_tag_button.clicked.connect(lambda _: self.sg_add_simple_button_clicked.emit(self.current_file_path, self.add_simple_tag_text_edit.text()))
        self.add_simple_tag_button.setIcon(QIcon.fromTheme(QIcon.ThemeIcon.ListAdd))
        # hbox_simple.addWidget(self.add_simple_tag_text_edit)
        # hbox_simple.addWidget(self.add_simple_tag_button)

        hbox_kv = QHBoxLayout()
        self.add_kv_tag_k_text_edit = QLineEdit()
        self.add_kv_tag_k_text_edit.setPlaceholderText("Key...")
        self.add_kv_tag_v_text_edit = QLineEdit()
        self.add_kv_tag_v_text_edit.setPlaceholderText("Value...")
        self.add_kv_tag_button = QPushButton("Add Key-Value Tag")
        self.add_kv_tag_button.setIcon(QIcon.fromTheme(QIcon.ThemeIcon.ListAdd))
        self.add_kv_tag_button.clicked.connect(lambda _: self.sg_add_kv_button_clicked.emit(self.current_file_path, self.add_kv_tag_k_text_edit.text(), self.add_kv_tag_v_text_edit.text()))
        hbox_kv.addWidget(self.add_kv_tag_k_text_edit)
        hbox_kv.addWidget(self.add_kv_tag_v_text_edit)

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
        layout.addLayout(self.vbox_tags)
        layout.addWidget(self.add_simple_tag_text_edit)
        layout.addWidget(self.add_simple_tag_button)
        layout.addLayout(hbox_kv, stretch=0)
        layout.addWidget(self.add_kv_tag_button)
        self.rebuild_tag_list()

        self.setLayout(layout)
        return
    
    def set_selected(self, fs_model: FileExplorerModel, index: QModelIndex, tag_model: TagModel):
        if not index.isValid():
            self.hide()
            return
        self.label_icon.setPixmap(fs_model.fileIcon(index).pixmap(32,32))
        self.label_name.setText(f"Name: {fs_model.fileName(index)}")
        self.current_file_path = fs_model.filePath(index)
        self.label_path.setText(f"Path: {fs_model.filePath(index)}")
        self.label_size.setText(f"Size: {fs_model.size(index)}")
        lm = fs_model.lastModified(index)
        self.label_last_modified.setText(f"Last Modified: {fs_model.lastModified(index).toString()}")

        # self.tags = tag_model.get_tags_for_filename(fs_model.filePath(index));
        self.tags = tag_model.get_tags_for_filename_as_list(fs_model.filePath(index))
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
                label = QLabel()
                delete_btn = QPushButton()
                delete_btn.setIcon(QIcon.fromTheme(QIcon.ThemeIcon.ListRemove))
                if isinstance(tag, list) and len(tag) >= 2:
                    label.setText(f"{tag[0]}: {tag[1]}")
                    delete_btn.setProperty("tag_t1", tag[0])
                    delete_btn.setProperty("tag_t2", tag[1])
                else:
                    label.setText(tag) # ignore warning, we know it's string # type: ignore
                    delete_btn.setProperty("tag_t1", tag)
                    delete_btn.setProperty("tag_t2", None)
                delete_btn.clicked.connect(lambda _, button=delete_btn: self.sg_remove_tab_button_clicked.emit(self.current_file_path, button.property("tag_t1"), button.property("tag_t2")))
                layout.addWidget(label)
                layout.addStretch()
                layout.addWidget(delete_btn)
                self.vbox_tags.addWidget(row)

class QueryTab(QWidget):
    def __init__(self, fs_model: FileExplorerModel, tag_model: TagModel):
        super().__init__()
        self.tag_model = tag_model
        layout = QVBoxLayout(self)

        # ** LEFT - SEARCH BOX ** #
        self.left_root = QuerySearchArea()        
        # ** MIDDLE - FILES ** #
        self.middle_root = QLabel("Mid")
        # ** RIGHT - FILE INFO ** #
        # self.right_root = FileInfoWidget()

        self.right_root_stack = QStackedWidget()
        scroll_area = QScrollArea()
        scroll_area.setWidgetResizable(True)
        scroll_area.setHorizontalScrollBarPolicy(Qt.ScrollBarPolicy.ScrollBarAlwaysOff)
        scroll_area.setVerticalScrollBarPolicy(Qt.ScrollBarPolicy.ScrollBarAsNeeded)
        self.right_file_info_widget = FileInfoWidget()
        self.right_placeholder_widget = QLabel("Select a file...")
        self.right_placeholder_widget.setAlignment(Qt.AlignmentFlag.AlignHCenter | Qt.AlignmentFlag.AlignVCenter)
        self.right_root_stack.addWidget(self.right_placeholder_widget) # Index 0
        self.right_root_stack.addWidget(self.right_file_info_widget) # Index 1
        self.right_root_stack.setSizePolicy(QSizePolicy.Policy.Expanding, QSizePolicy.Policy.MinimumExpanding)
        scroll_area.resizeEvent = self._on_scroll_resize
        scroll_area.setWidget(self.right_root_stack)
        self.right_root_stack.setCurrentIndex(0)

        # ** SPLITTER ROOT ** #
        splitter_root = QSplitter(Qt.Orientation.Horizontal)
        splitter_root.addWidget(self.left_root)
        splitter_root.addWidget(self.middle_root)
        splitter_root.addWidget(self.right_root_stack)
        layout.addWidget(splitter_root)
        splitter_root.setSizes([50,250,50])
    
    def _on_scroll_resize(self, event):
        self.right_root_stack.setMinimumWidth(event.size().width())
        event.accept()

class QuerySearchArea(QWidget):
    def __init__(self):
        super().__init__()
        left_root_layout = QVBoxLayout(self)
        left_root_layout.setAlignment(Qt.AlignmentFlag.AlignTop)

        panel_title_bar = QLabel("Tag Search")
        panel_title_bar.setAlignment(Qt.AlignmentFlag.AlignHCenter)
        left_root_layout.addWidget(panel_title_bar)
        tag_name_search = QLineEdit()
        tag_name_search.setPlaceholderText("Search...")
        left_root_layout.addWidget(tag_name_search)
        
        exact_fuzzy_group = QGroupBox()
        exact_fuzzy_group_layout = QHBoxLayout()
        exact_radio_btn = QRadioButton("Exact")
        fuzzy_radio_btn = QRadioButton("Fuzzy")
        exact_fuzzy_group_layout.addWidget(exact_radio_btn)
        exact_fuzzy_group_layout.addWidget(fuzzy_radio_btn)
        exact_fuzzy_group.setLayout(exact_fuzzy_group_layout)
        left_root_layout.addWidget(exact_fuzzy_group)

        flags_group = QGroupBox()
        flags_layout = QHBoxLayout()
        flags_label = QLabel("Include: ")
        flags_layout.addWidget(flags_label)
        flags_vbox = QVBoxLayout()
        checkbox_simple = QCheckBox()
        checkbox_simple.setText("Simple")
        checkbox_key = QCheckBox()
        checkbox_key.setText("Key")
        checkbox_value = QCheckBox()
        checkbox_value.setText("Value")
        flags_vbox.addWidget(checkbox_simple)
        flags_vbox.addWidget(checkbox_key)
        flags_vbox.addWidget(checkbox_value)
        flags_layout.addLayout(flags_vbox)
        flags_group.setLayout(flags_layout)
        left_root_layout.addWidget(flags_group)