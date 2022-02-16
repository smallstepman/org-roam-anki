;;; package --- Summary
;;; Commentary:
;;;
;;; Code:
(defcustom org-roam-anki-sync-should-block t
  "doc"
  :group 'checkbox
  :type '(repeat string))

(defcustom org-roam-anki-path-to-binary (expand-file-name "~/.cargo/bin/org-roam-anki")
  "doc"
  :group 'checkbox
  :type '(repeat string))

(defun org-roam-anki-sync-file (path)
  "sync a file"
  (let ((command (concat org-roam-anki-path-to-binary " ")))
    (if org-roam-anki-sync-should-block
        (start-process-shell-command "" nil command)
      (shell-command-to-string command))))

(provide 'org-roam-anki)
;;; org-roam-anki.el ends here
