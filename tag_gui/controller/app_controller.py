from model.file_explorer_model import FileExplorerModel
from model.tag_model import TagModel
from model.file_query_model import FileQueryModel
from view.main_window import FileInfoWidget, FilesTab, QueryTab

class AppController():
    def __init__(self, fs_model: FileExplorerModel, tag_model: TagModel, file_info_widget: FileInfoWidget, files_tab: FilesTab, query_tab: QueryTab, file_query_model: FileQueryModel):
        self.fs_model = fs_model
        self.tag_model = tag_model
        self.file_query_model = file_query_model
        self.file_info_widget = file_info_widget
        self.files_tab = files_tab
        self.query_tab = query_tab

        self.file_info_widget.sg_remove_tab_button_clicked.connect(self._on_tag_btn_delete_click)
        self.file_info_widget.sg_add_simple_button_clicked.connect(self._on_tag_simple_btn_add_click)
        self.file_info_widget.sg_add_kv_button_clicked.connect(self._on_tag_kv_btn_add_click)

        # ** Query Tab ** #
        self.query_tab.middle_root.setModel(self.file_query_model)
        self.query_tab.left_root.sg_search_query_entered.connect(self._on_query_entered)
        self.query_tab.sg_file_folder_double_click.connect(self._on_queryview_doubleclick)

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
            self.file_query_model.rebuild_from_mapping()
    
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
            self.file_query_model.rebuild_from_mapping()
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
            self.file_query_model.rebuild_from_mapping()
        return

    def _on_query_entered(self, exact: bool, text: str, simple: bool, key: bool, value: bool):
        self.file_query_model.mapping = self.tag_model.do_query(exact, text, simple, key, value)
        self.file_query_model.rebuild_from_mapping()
        return
    
    def _on_queryview_doubleclick(self, index):
        self.file_query_model.open_file_info_from_index(index)