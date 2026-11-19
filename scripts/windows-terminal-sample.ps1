$esc = [char]27
Write-Host 'LBRX — 한글 글꼴과 자간을 확인합니다. The quick brown fox. 0O 1l {} []'
Write-Host 'Drag to select this line / 이 줄을 마우스로 선택해 보세요.'
foreach ($offset in 0, 8) {
    $swatches = for ($slot = 0; $slot -lt 8; $slot++) {
        $background = 40 + $slot + $(if ($offset -eq 8) { 60 } else { 0 })
        '{0}[{1}m    {0}[0m {2,2}' -f $esc, $background, ($offset + $slot)
    }
    Write-Host ($swatches -join '  ')
}
Write-Host 'The cursor at the prompt uses the theme cursor color.'
