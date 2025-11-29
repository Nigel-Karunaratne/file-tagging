from model.file_explorer_model import FileExplorerModel
from model.tag_model import TagModel
from view.main_window import FileInfoWidget, FilesTab

class AppController():
    def __init__(self, fs_model: FileExplorerModel, tag_model: TagModel, file_info_widget: FileInfoWidget, files_tab: FilesTab):
        self.fs_model = fs_model
        self.tag_model = tag_model
        self.file_info_widget = file_info_widget
        self.files_tab = files_tab

        self.file_info_widget.sg_remove_tab_button_clicked.connect(self._on_tag_btn_delete_click)
        self.file_info_widget.sg_add_simple_button_clicked.connect(self._on_tag_simple_btn_add_click)
        self.file_info_widget.sg_add_kv_button_clicked.connect(self._on_tag_kv_btn_add_click)

    def _on_tag_btn_delete_click(self, file_name, tag_t1, tag_t2):
        print(f"tag is {file_name} || {tag_t1} | {tag_t2}")
        rv = self.tag_model.remove_tag_from_file(file_name, tag_t1, tag_t2)
        print(f"RV IS {rv}")
        if rv: # If removing the tag was successful, we need to re-fresh the views manually. Signals would have helped...
            # Refresh the file explorer model, by resetting its root dir and setting it back again
            self.fs_model.set_directory(self.fs_model.current_directory, self.files_tab.tag_model)
            root_path = self.fs_model.rootPath()
            self.fs_model.setRootPath("")
            self.fs_model.setRootPath(root_path)

            # Refresh the file info widget
            self.file_info_widget.tags = self.tag_model.get_tags_for_filename_as_list(self.file_info_widget.current_file_path)
            self.file_info_widget.rebuild_tag_list()
            self.file_info_widget.update()
    
    def _on_tag_simple_btn_add_click(self, file_name, tag_t1):
        rv = self.tag_model.add_tag_to_file(file_name, tag_t1, None)
        if rv:
            self.fs_model.set_directory(self.fs_model.current_directory, self.files_tab.tag_model)
            root_path = self.fs_model.rootPath()
            self.fs_model.setRootPath("")
            self.fs_model.setRootPath(root_path)

            # Refresh the file info widget
            self.file_info_widget.tags = self.tag_model.get_tags_for_filename_as_list(self.file_info_widget.current_file_path)
            self.file_info_widget.rebuild_tag_list()
            self.file_info_widget.update()
        return
    
    def _on_tag_kv_btn_add_click(self, file_name, tag_t1, tag_t2):
        rv = self.tag_model.add_tag_to_file(file_name, tag_t1, tag_t2)
        if rv:
            self.fs_model.set_directory(self.fs_model.current_directory, self.files_tab.tag_model)
            root_path = self.fs_model.rootPath()
            self.fs_model.setRootPath("")
            self.fs_model.setRootPath(root_path)

            # Refresh the file info widget
            self.file_info_widget.tags = self.tag_model.get_tags_for_filename_as_list(self.file_info_widget.current_file_path)
            self.file_info_widget.rebuild_tag_list()
            self.file_info_widget.update()
        return

    
