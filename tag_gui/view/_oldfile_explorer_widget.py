# OLD file_explorer_widget.py, kept for reference

import os
from PySide6.QtCore import Qt
from PySide6.QtWidgets import QWidget, QVBoxLayout, QTreeView, QLineEdit, QFileDialog, QPushButton
from model.file_explorer_model import FileExplorerModel  # Import the custom model class

class FileExplorerWidget(QWidget):
    def __init__(self):
        super().__init__()

        self.setWindowTitle("File Explorer")
        self.setGeometry(100, 100, 800, 600)

        self.current_dir = os.path.expanduser("~")  # Start at the home directory

        internal_layout = QVBoxLayout()
        
        # Address bar at the top
        self.address_bar = QLineEdit(self)
        self.address_bar.setText(self.current_dir)
        self.address_bar.returnPressed.connect(self.on_address_change)
        internal_layout.addWidget(self.address_bar)

        # File Tree View
        self.tree_view = QTreeView(self)
        self.tree_view.setRootIsDecorated(True)
        self.tree_view.setSortingEnabled(True)
        internal_layout.addWidget(self.tree_view)

        # Load initial directory
        self.load_directory(self.current_dir)

        # Buttons for navigation
        self.nav_button = QPushButton("Open Folder", self)
        self.nav_button.clicked.connect(self.open_folder)
        internal_layout.addWidget(self.nav_button)

        self.setLayout(internal_layout)

    def load_directory(self, path: str):
        """Load the directory contents into the tree view."""
        self.current_dir = path
        self.address_bar.setText(path)
        
        # Create a list of directories and files with tags
        directories, files = self.get_directory_contents(path)
        
        self.file_system = []
        self.file_system.append(("..", os.path.dirname(path), ""))  # Parent folder with no tag
        self.file_system.extend([(d, os.path.join(path, d), "Folder") for d in directories])
        self.file_system.extend([(f, os.path.join(path, f), "No Tag") for f in files])

        self.tree_view.setModel(FileExplorerModel(self.file_system))  # Use the imported model
        self.tree_view.expandAll()

    def get_directory_contents(self, path: str):
        """Return the directories and files in a given path."""
        try:
            dirs, files = [], []
            with os.scandir(path) as it:
                for entry in it:
                    if entry.is_dir():
                        dirs.append(entry.name)
                    elif entry.is_file():
                        files.append(entry.name)
            return dirs, files
        except FileNotFoundError:
            return [], []

    def on_address_change(self):
        """Handle address bar input."""
        new_dir = self.address_bar.text()
        if os.path.isdir(new_dir):
            self.load_directory(new_dir)

    def open_folder(self):
        """Open a dialog to choose a folder."""
        folder = QFileDialog.getExistingDirectory(self, "Select Directory", self.current_dir)
        if folder:
            self.load_directory(folder)
