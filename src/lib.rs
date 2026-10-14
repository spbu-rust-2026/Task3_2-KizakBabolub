pub fn add_command(history: &mut std::collections::VecDeque<String>, command: String, k: usize) {
    if k == 0 {
        history.clear();
        return;
    }

    if history.len() >= k {
        history.pop_front();
    }

    history.push_back(command);
}
