from model.file_explorer_model import FileExplorerModel
from model.tag_model import TagModel
from model.file_query_model import FileQueryModel
from view.main_window import MainWindow, FileInfoWidget, FilesTab, QueryTab

from PySide6.QtWidgets import QApplication

class AppController():
    def __init__(self, app: QApplication, fs_model: FileExplorerModel, tag_model: TagModel, file_query_model: FileQueryModel, view: MainWindow):
        self.fs_model = fs_model
        self.tag_model = tag_model
        self.file_query_model = file_query_model
        self.explore_file_info_widget = view.files_tab.right_file_info_widget
        self.query_file_info_widget = view.query_tab.right_file_info_widget
        self.files_tab = view.files_tab
        self.query_tab = view.query_tab

        self.main_window = view

        self.fs_model.set_directory(self.fs_model.current_directory, self.tag_model.get_tag_mapping_in_dir_as_strings(self.fs_model.current_directory))

        view.sg_quit_app_request.connect(lambda: app.quit())
        view.sg_create_workspace_request.connect(self._on_mainwindow_create_workspace_request)
        view.sg_open_workspace_request.connect(self._on_mainwindow_open_workspace_request)

        # ** Explore Tab ** #
        self.files_tab.left_file_hierarchy.setModel(self.fs_model)
        self.files_tab.left_file_hierarchy.setRootIndex(self.fs_model.index(self.fs_model.current_directory))
        for col in range(2, self.fs_model.columnCount()):
            self.files_tab.left_file_hierarchy.setColumnHidden(col, True)
        self.files_tab.left_file_hierarchy.selectionModel().selectionChanged.connect(self.files_tab.on_file_folder_selection_changed)

        # Force a name change manually
        self.files_tab.set_workspace_name_label(self.tag_model.get_workspace_name())
        self.tag_model.sg_workspace_name_change.connect(self.files_tab.set_workspace_name_label)
        self.fs_model.sg_going_up_directory.connect(self._on_fsmodel_directorychange)

        self.files_tab.sg_file_folder_doubleclick.connect(self._on_explorerview_doubleclick)
        self.files_tab.sg_directory_up_btn_click.connect(self._on_exploreview_upbtn_click)
        self.files_tab.sg_selected_file_change.connect(self._on_exploreview_selected_file_change)

        self.explore_file_info_widget.sg_remove_tab_button_clicked.connect(self._on_tag_btn_delete_click)
        self.explore_file_info_widget.sg_add_simple_button_clicked.connect(self._on_tag_simple_btn_add_click)
        self.explore_file_info_widget.sg_add_kv_button_clicked.connect(self._on_tag_kv_btn_add_click)

        # ** Query Tab ** #
        self.query_tab.middle_root.setModel(self.file_query_model)
        self.query_tab.left_root.sg_search_query_entered.connect(self._on_query_entered)
        self.query_tab.sg_file_folder_double_click.connect(self._on_queryview_doubleclick)

        self.query_tab.middle_root.selectionModel().selectionChanged.connect(self.query_tab.on_file_folder_selection_changed)
        self.query_tab.sg_selected_file_change.connect(self._on_queryview_selected_file_change)

        self.query_file_info_widget.sg_remove_tab_button_clicked.connect(self._on_tag_btn_delete_click)
        self.query_file_info_widget.sg_add_simple_button_clicked.connect(self._on_tag_simple_btn_add_click)
        self.query_file_info_widget.sg_add_kv_button_clicked.connect(self._on_tag_kv_btn_add_click)

    def _on_tag_btn_delete_click(self, file_name, tag_t1, tag_t2):
        # print(f"tag is {file_name} || {tag_t1} | {tag_t2}")
        rv = self.tag_model.remove_tag_from_file(file_name, tag_t1, tag_t2)
        # print(f"RV IS {rv}")
        if rv: # If removing the tag was successful, we need to re-fresh the views manually. Signals would have helped...
            # Refresh the file explorer model, by resetting its root dir and setting it back again
            self.fs_model.set_directory(self.fs_model.current_directory, self.tag_model.get_tag_mapping_in_dir_as_strings(self.fs_model.current_directory))
            root_path = self.fs_model.rootPath()
            self.fs_model.setRootPath("")
            self.fs_model.setRootPath(root_path)

            # Refresh the file info widget(s)
            self.refresh_info_widgets_after_add_or_del()
    
    def _on_tag_simple_btn_add_click(self, file_name, tag_t1):
        rv = self.tag_model.add_tag_to_file(file_name, tag_t1, None)
        if rv:
            self.fs_model.set_directory(self.fs_model.current_directory, self.tag_model.get_tag_mapping_in_dir_as_strings(self.fs_model.current_directory))
            root_path = self.fs_model.rootPath()
            self.fs_model.setRootPath("")
            self.fs_model.setRootPath(root_path)

            # Refresh the file info widget(s)
            self.refresh_info_widgets_after_add_or_del()
        return
    
    def _on_tag_kv_btn_add_click(self, file_name, tag_t1, tag_t2):
        rv = self.tag_model.add_tag_to_file(file_name, tag_t1, tag_t2)
        if rv:
            self.fs_model.set_directory(self.fs_model.current_directory, self.tag_model.get_tag_mapping_in_dir_as_strings(self.fs_model.current_directory))
            root_path = self.fs_model.rootPath()
            self.fs_model.setRootPath("")
            self.fs_model.setRootPath(root_path)

            # Refresh the file info widget(s)
            self.refresh_info_widgets_after_add_or_del()
        return

    def refresh_info_widgets_after_add_or_del(self):
            self.explore_file_info_widget.tags = self.tag_model.get_tags_for_filename_as_list(self.explore_file_info_widget.current_file_path)
            self.explore_file_info_widget.rebuild_tag_list()
            self.explore_file_info_widget.update()

            if self.query_file_info_widget.current_file_path != "":
                self.query_file_info_widget.tags = self.tag_model.get_tags_for_filename_as_list(self.query_file_info_widget.current_file_path)
                self.file_query_model.mapping = self.tag_model.do_query(self.file_query_model.last_query_params[0], self.file_query_model.last_query_params[1], self.file_query_model.last_query_params[2], self.file_query_model.last_query_params[3], self.file_query_model.last_query_params[4])
                self.query_file_info_widget.rebuild_tag_list()
                self.query_file_info_widget.update()
                self.file_query_model.rebuild_from_mapping()

    def _on_query_entered(self, exact: bool, text: str, simple: bool, key: bool, value: bool):
        self.file_query_model.mapping = self.tag_model.do_query(exact, text, simple, key, value)
        self.file_query_model.set_last_query_params(exact, text, simple, key, value)
        self.file_query_model.rebuild_from_mapping()
        return
    
    def _on_explorerview_doubleclick(self, index):
        print("something was double clicked")
        is_file, dir_path = self.fs_model.open_file_info_from_index(index)
        if not is_file:
            self.fs_model.set_directory(dir_path, self.tag_model.get_tag_mapping_in_dir_as_strings(dir_path))
            self.files_tab.left_file_hierarchy.setRootIndex(self.fs_model.index(dir_path))

            self.files_tab.set_info_to_placeholder()
    
    def _on_exploreview_upbtn_click(self):
        self.fs_model.attempt_to_go_up()
        return
    
    def _on_fsmodel_directorychange(self, parent_dir: str):
        mapping = self.tag_model.get_tag_mapping_in_dir_as_strings(parent_dir)
        print(f"mapping is {mapping}")
        self.fs_model.set_directory(parent_dir, mapping)
        self.files_tab.left_file_hierarchy.setRootIndex(self.fs_model.index(parent_dir))

        self.files_tab.set_info_to_placeholder()
        return

    def _on_exploreview_selected_file_change(self, index):
        self.explore_file_info_widget.set_selected(index, self.fs_model.fileIcon(index), self.fs_model.fileName(index), self.fs_model.filePath(index), self.fs_model.size(index), self.fs_model.lastModified(index).toString())
        self.explore_file_info_widget.tags = self.tag_model.get_tags_for_filename_as_list(self.fs_model.filePath(index))
        self.explore_file_info_widget.rebuild_tag_list()
        self.explore_file_info_widget.show()
        return
    
    def _on_queryview_selected_file_change(self, index):
        print("WE ARE HERE!")
        info = self.file_query_model.get_file_info_from_index(index)
        if info == None:
            return
        icon = self.file_query_model.get_icon_from_info(info)
        self.query_file_info_widget.current_file_path = info.filePath()
        self.query_file_info_widget.set_selected(index, icon, info.fileName(), info.filePath(), info.size(), info.lastModified().toString())
        self.query_file_info_widget.tags = self.tag_model.get_tags_for_filename_as_list(info.filePath())
        self.query_file_info_widget.rebuild_tag_list()
        self.query_file_info_widget.show()
        return

    def _on_queryview_doubleclick(self, index):
        self.file_query_model.open_file_info_from_index(index)

    def _on_mainwindow_create_workspace_request(self, requested_name: str):
        try:
            self.tag_model.create_and_set_workspace(self.fs_model.current_directory, requested_name)
        except Exception:
            self.main_window.show_workspace_action_fail_message("Cannot create workspace", "The workspace could not be created.")
            return
        self.files_tab.set_info_to_placeholder()

        # Refresh tags for file explorer model
        mapping = self.tag_model.get_tag_mapping_in_dir_as_strings(self.tag_model.cwd)
        self.fs_model.set_directory(self.fs_model.current_directory, mapping)
        self.files_tab.left_file_hierarchy.setRootIndex(self.fs_model.index(self.fs_model.current_directory))

        self.file_query_model.clear()
        return

    def _on_mainwindow_open_workspace_request(self, requested_name: str):
        try:
            self.tag_model.open_and_set_workspace(self.fs_model.current_directory, requested_name)
        except Exception:
            self.main_window.show_workspace_action_fail_message("Cannot open workspace", "The workspace could not be opened.")
            return
        self.files_tab.set_info_to_placeholder()

        mapping = self.tag_model.get_tag_mapping_in_dir_as_strings(self.tag_model.cwd)
        self.fs_model.set_directory(self.fs_model.current_directory, mapping)
        self.files_tab.left_file_hierarchy.setRootIndex(self.fs_model.index(self.fs_model.current_directory))

        self.file_query_model.clear()
        return