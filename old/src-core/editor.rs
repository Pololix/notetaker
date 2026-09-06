pub fn set_viewport(&mut self, viewport: Viewport) -> Result<(), EditorError> {
    self.viewport = viewport;
    self.get_mut_workspace(self.active_id)?
        .adapt_to_viewport(viewport)?;

    Ok(())
}

pub fn render(&self) -> Result<RenderFrame, EditorError> {
    let active = self.get_workspace(self.active_id)?;
    let cmds = active.render();

    Ok(RenderFrame { cmds })
}

fn add_workspace(&mut self, viewport: Viewport) -> Result</*EditorEvent*/ (), EditorError> {
    if self.workspace_count > 9 {
        return Err(EditorError::Overflow);
    }

    let new_workspace = Workspace::new(viewport)?;
    self.workspaces.insert(self.next_id, new_workspace);
    self.workspace_count += 1;
    self.next_id += 1;

    // Ok(EditorEvent::WorkspaceCreated)
    Ok(())
}

fn delete_active(&mut self) // -> EditorEvent
{
    self.workspaces.remove(&self.active_id);
    self.workspace_count -= 1;

    if self.workspace_count == 0 {
        panic!(); // for now
    }

    // EditorEvent::WorkspaceDeleted
}

fn get_workspace(&self, id: WorkspaceId) -> Result<&Workspace, EditorError> {
    match self.workspaces.get(&id) {
        Some(workspace) => Ok(workspace),
        None => return Err(EditorError::NullActive),
    }
}

fn get_mut_workspace(&mut self, id: WorkspaceId) -> Result<&mut Workspace, EditorError> {
    match self.workspaces.get_mut(&id) {
        Some(workspace) => Ok(workspace),
        None => return Err(EditorError::NullActive),
    }
}
