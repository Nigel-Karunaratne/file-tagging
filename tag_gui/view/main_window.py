from PySide6.QtCore import Qt, QDir, QModelIndex, Signal
from PySide6.QtGui import QIcon, QAction
from PySide6.QtWidgets import (
    QWidget, QTabWidget, QVBoxLayout, QHBoxLayout, QLabel, QSplitter, QFileSystemModel, QTreeView, QMenuBar, QHeaderView, QPushButton, QStackedWidget, QLineEdit, QScrollArea, QSizePolicy, QCheckBox, QRadioButton, QGroupBox
)

class MainWindow(QWidget):
    def __init__(self):
        super().__init__()
        self.setWindowTitle("File Tag GUI")

        layout = QVBoxLayout(self)
        
        menu_bar = self._make_menu_bar()
        layout.addWidget(menu_bar)
        self.tabs = QTabWidget()

        self.files_tab = FilesTab()
        self.tabs.addTab(self.files_tab, "Files")

        # Tab 2 (tag explorer)
        self.query_tab = QueryTab() 
        self.tabs.addTab(self.query_tab, "Search")

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
    sg_file_folder_doubleclick = Signal(QModelIndex)
    sg_directory_up_btn_click = Signal()
    sg_selected_file_change = Signal(QModelIndex)

    def __init__(self):
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
        # self.set_workspace_name_label(tag_model)
        nav_bar.addWidget(self.workspace_name_label)
        layout.addLayout(nav_bar)
        
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
        self.left_file_hierarchy.setItemsExpandable(False)
        self.left_file_hierarchy.setRootIsDecorated(False)
        header = self.left_file_hierarchy.header()
        header.setSectionResizeMode(0,QHeaderView.ResizeMode.Interactive)
        header.setSectionResizeMode(1,QHeaderView.ResizeMode.ResizeToContents)
        header.setMinimumSectionSize(50)
        self.left_file_hierarchy.setColumnWidth(0, 250)
        self.left_file_hierarchy.doubleClicked.connect(self._on_file_folder_double_click)

        # ** ROOT SPLITTER ** #
        splitter_root = QSplitter(Qt.Orientation.Horizontal)

        splitter_root.addWidget(self.left_file_hierarchy)
        splitter_root.addWidget(scroll_area)

        layout.addWidget(splitter_root)
        splitter_root.setSizes([250,40])

    def set_workspace_name_label(self, new_name):
        if new_name == "":
            self.workspace_name_label.setText("No Workspace Opened")
        else:
            self.workspace_name_label.setText(f"Workspace: {new_name}")

    def on_scroll_resize(self, event):
        self.right_stack.setMinimumWidth(event.size().width())
        event.accept()

    def on_file_folder_selection_changed(self, selected, _deselected):
        indexes = selected.indexes()
        if not indexes or not indexes[0].isValid():
            self.right_stack.setCurrentIndex(0)
        else:
            self.right_stack.setCurrentIndex(1)
            self.sg_selected_file_change.emit(indexes[0].siblingAtColumn(0))
        return

    def _on_file_folder_double_click(self, index: QModelIndex):
        self.sg_file_folder_doubleclick.emit(index)
        return
    
    def _on_directory_up_btn_clicked(self):
        self.sg_directory_up_btn_click.emit()
        return
        

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
    
    def set_selected(self, index: QModelIndex, icon: QIcon, fname: str, path: str, size: int, last_modified: str):
        if not index.isValid():
            self.hide()
            return
        self.label_icon.setPixmap(icon.pixmap(32,32))
        self.label_name.setText(f"Name: {fname}")
        self.current_file_path = path
        self.label_path.setText(f"Path: {path}")
        self.label_size.setText(f"Size: {size}")
        self.label_last_modified.setText(f"Last Modified: {last_modified}")
        # Setting tags and showing widget is done in controller
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
    sg_file_folder_double_click = Signal(QModelIndex)
    sg_selected_file_change = Signal(QModelIndex)

    def __init__(self):
        super().__init__()
        layout = QVBoxLayout(self)

        # ** LEFT - SEARCH BOX ** #
        self.left_root = QuerySearchArea()        
        # ** MIDDLE - FILES ** #
        self.middle_root = QTreeView()
        self.middle_root.setItemsExpandable(False)
        self.middle_root.setRootIsDecorated(False)
        header = self.middle_root.header()
        header.setSectionResizeMode(0,QHeaderView.ResizeMode.Interactive)
        header.setSectionResizeMode(1,QHeaderView.ResizeMode.ResizeToContents)
        header.setMinimumSectionSize(50)
        self.middle_root.doubleClicked.connect(self._on_file_folder_double_click)
        # ** RIGHT - FILE INFO ** #
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

    def _on_file_folder_double_click(self, index: QModelIndex):
        self.sg_file_folder_double_click.emit(index)
        return
    
    def _on_scroll_resize(self, event):
        self.right_root_stack.setMinimumWidth(event.size().width())
        event.accept()

    def on_file_folder_selection_changed(self, selected, _deselected):
        indexes = selected.indexes()
        if not indexes or not indexes[0].isValid():
            self.right_root_stack.setCurrentIndex(0)
        else:
            self.right_root_stack.setCurrentIndex(1)
            self.sg_selected_file_change.emit(indexes[0].siblingAtColumn(0))
        return

class QuerySearchArea(QWidget):
    sg_search_query_entered = Signal(bool, str, bool, bool, bool)

    def __init__(self):
        super().__init__()
        left_root_layout = QVBoxLayout(self)
        left_root_layout.setAlignment(Qt.AlignmentFlag.AlignTop)

        panel_title_bar = QLabel("Tag Search")
        panel_title_bar.setAlignment(Qt.AlignmentFlag.AlignHCenter)
        left_root_layout.addWidget(panel_title_bar)
        self.tag_name_search = QLineEdit()
        self.tag_name_search.setPlaceholderText("Search by Tags...")
        left_root_layout.addWidget(self.tag_name_search)
        
        exact_fuzzy_group = QGroupBox()
        exact_fuzzy_group_layout = QHBoxLayout()
        self.exact_radio_btn = QRadioButton("Exact")
        self.exact_radio_btn.setChecked(True)
        self.fuzzy_radio_btn = QRadioButton("Fuzzy")
        exact_fuzzy_group_layout.addWidget(self.exact_radio_btn)
        exact_fuzzy_group_layout.addWidget(self.fuzzy_radio_btn)
        exact_fuzzy_group.setLayout(exact_fuzzy_group_layout)
        left_root_layout.addWidget(exact_fuzzy_group)

        flags_group = QGroupBox()
        flags_layout = QHBoxLayout()
        flags_label = QLabel("Include: ")
        flags_layout.addWidget(flags_label)
        flags_vbox = QVBoxLayout()
        self.checkbox_simple = QCheckBox()
        self.checkbox_simple.setText("Simple")
        self.checkbox_simple.setChecked(True)
        self.checkbox_key = QCheckBox()
        self.checkbox_key.setText("Key")
        self.checkbox_key.setChecked(True)
        self.checkbox_value = QCheckBox()
        self.checkbox_value.setText("Value")
        self.checkbox_value.setChecked(True)
        flags_vbox.addWidget(self.checkbox_simple)
        flags_vbox.addWidget(self.checkbox_key)
        flags_vbox.addWidget(self.checkbox_value)
        flags_layout.addLayout(flags_vbox)
        flags_group.setLayout(flags_layout)
        left_root_layout.addWidget(flags_group)

        self.tag_name_search.editingFinished.connect(self._emit_search_query_entered)
    
    def _emit_search_query_entered(self):
        exact = self.exact_radio_btn.isChecked()
        text = self.tag_name_search.text()
        simple = self.checkbox_simple.isChecked()
        key = self.checkbox_key.isChecked()
        value = self.checkbox_value.isChecked()
        self.sg_search_query_entered.emit(exact, text, simple, key, value)
        return