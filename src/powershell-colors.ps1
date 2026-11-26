# LBRX optional PowerShell 7.2+ directory colors. Dot-source explicitly; does not write a profile.
if ($PSVersionTable.PSVersion -lt [version]'7.2') {
    throw 'LBRX directory colors require PowerShell 7.2 or newer.'
}
$PSStyle.FileInfo.Directory = "$([char]27)[96;1m"
