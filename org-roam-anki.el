;;; package --- Summary
;;; Commentary:
;;;
;;; Code:
(defcustom org-roam-anki-sync-should-block t
  "doc"
  :group 'org-roam-anki
  :type '(boolean))

(defcustom org-roam-anki-path-to-binary (expand-file-name "~/.cargo/bin/org-roam-anki")
  "doc"
  :group 'org-roam-anki
  :type '(string))

(defcustom org-roam-anki-ignore-dailies t
  "doc"
  :group 'org-roam-anki
  :type '(boolean))

(defcustom org-roam-anki-filter-tags '()
  "doc"
  :group 'org-roam-anki
  :type '((list string)))

(defcustom org-roam-anki-filter-directories '()
  "doc"
  :group 'org-roam-anki
  :type '((list string)))

(defun org-roam-anki--compute-options ()
  "compute options based on org-roam-anki-filter-... values"
  "")

(defun org-roam-anki--send-file-to-sync (path)
  "sync a file"
  (let* ((binary org-roam-anki-path-to-binary)
         (options (org-roam-anki--compute-options))
         (command (concat binary path options)))
    (if org-roam-anki-sync-should-block
        (start-process-shell-command "" nil command)
      (shell-command-to-string command))))

;;; Public API
;;;###autoload
(defun org-roam-anki-sync-all-nodes ()
  "sync all nodes, respecting filter-tags/-directories settings
   should iterate over nodes, filter out stuff, and send path to
   org-roam-anki--send-file-to-sync"
  (interactive)
  "todo")
;;;###autoload
(defun org-roam-anki-sync-section ()
  "sync section based on where the cursor is currently place"
  (interactive)
  "todo")
;;;###autoload
(defun org-roam-anki-sync-file ()
  "sync currently active file"
  (interactive)
  (org-roam-anki--send-file-to-sync (file-name-directory (buffer-file-name))))

(provide 'org-roam-anki)
;;; org-roam-anki.el ends here
