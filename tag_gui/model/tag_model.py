import rs_tags as tags
import os

from typing import Dict

from PySide6.QtCore import Signal, QObject

class TagModel(QObject):
    # sg_tag_info_changed = Signal()
    sg_workspace_name_change = Signal(str)

    def __init__(self, cwd):
        super().__init__()
        self.cwd = cwd
        self.current_workspace: tags.TagWorkspace | None = None

        self.current_workspace = tags.TagWorkspace.open_workspace(self.cwd, "test")
        if self.current_workspace:
            self.current_workspace.scan_for_tagfiles()
        print(cwd)

    def get_tag_mapping_in_dir_as_strings(self, path_to_directory: str) -> Dict[str, str]:
        # print(f"START get_tag_mapping_in_dir_as_strings: {path_to_directory}")
        if self.current_workspace == None:
            return {}
        return_val = {}
        for entry in os.listdir(path_to_directory):
            full_path = os.path.join(path_to_directory, entry)
            list_tag = self.current_workspace.get_tags_for_file_name(full_path)
            if len(list_tag) <= 0:
                continue
            tags_as_list = []
            for tag in list_tag:
                if tag.is_simple():
                    tags_as_list.append(tag.simple_value)
                else:
                    tags_as_list.append(f"{tag.kv_key}: {tag.kv_value}")
            print(tags_as_list)
            return_val[entry] = ", ".join(tags_as_list)
        
        # print(f"DONE get_tag_mapping_in_dir_as_strings: {path_to_directory}")
        return return_val
    
    def get_tags_for_filename_as_list(self, path_to_file_name: str) -> list:
        # print(f"START get_tags_for_filename_as_list_of_str: {path_to_file_name}")
        if self.current_workspace:
            list_tag = self.current_workspace.get_tags_for_file_name(path_to_file_name)
            if len(list_tag) <= 0:
                return []
            return_list = []
            for tag in list_tag:
                if tag.is_simple():
                    return_list.append(tag.simple_value)
                else:
                    return_list.append([tag.kv_key, tag.kv_value])
            # print(f"DONE get_tags_for_filename_as_list_of_str: {path_to_file_name}")
            return return_list
        else:
            return []
    
    def add_tag_to_file(self, path_to_file_name: str, tag1_to_add: str, tag2_to_add: str | None):
        if self.current_workspace == None:
            return False
        try:
            self.current_workspace.add_tag_to_file(path_to_file_name, tag1_to_add, tag2_to_add)
        except Exception:
            return False
        return True

    def remove_tag_from_file(self, path_to_file_name: str, tag1_to_remove: str, tag2_to_remove: str | None):
        if self.current_workspace == None:
            return False
        try:
            self.current_workspace.remove_tag_from_file(path_to_file_name, tag1_to_remove, tag2_to_remove)
        except Exception:
            return False
        return True
        
    def get_workspace_name(self) -> str:
        if self.current_workspace == None:
            return ""
        return self.current_workspace.get_name()
    
    def open_and_set_workspace(self, new_cwd, workspace_name) -> bool:
        wksp = tags.TagWorkspace.open_workspace(new_cwd, workspace_name)
        if wksp != None:
            print(f" NEW open workspace {workspace_name}")
            self.current_workspace = wksp
            self.cwd = new_cwd
            self.current_workspace.scan_for_tagfiles()
            self.sg_workspace_name_change.emit(workspace_name)
            return True
        return False
        
    
    def create_and_set_workspace(self, new_cwd, workspace_name) -> bool:
        wksp = tags.TagWorkspace.create_workspace(new_cwd, workspace_name)
        if wksp != None:
            print(f" NEW create workspace {workspace_name}")
            self.current_workspace = wksp
            self.cwd = new_cwd
            self.current_workspace.scan_for_tagfiles()
            self.sg_workspace_name_change.emit(workspace_name)
            return True
        return False
    
    def do_query(self, exact: bool, text: str, simple=True, key=True, value=True) -> dict:
        if self.current_workspace == None:
            return dict()
        rv = dict()
        qv = self.current_workspace.query_exact(text, simple, key, value) if exact else self.current_workspace.query_fuzzy(text, simple, key, value)

        for key, value in qv.items():
            tag_strs = []
            for tag in value:
                if tag.is_simple():
                    tag_strs.append(tag.simple_value)
                else:
                    tag_strs.append(f"{tag.kv_key}: {tag.kv_value}")
            rv[key] = ",".join(tag_strs)
        return rv