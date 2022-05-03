echo "running fixer"
cp -f $1/Contents/Frameworks/UnityPlayer.dylib /usr/local/lib/UnityPlayer.dylib || echo "unable to fix unity"
{
xattr -r -d com.apple.quarantine /usr/local/lib/UnityPlayer.dylib
xattr -r -d com.apple.quarantine $1/Contents/MacOS/Vulnus
} || echo "unable to mark as safe"
mv $1/Contents/MacOS/Vulnus $1
chmod +x $1/Vulnus
$loc = pwd
mv $1 ./vulnus-mac.app
xattr -r -d com.apple.quarantine ./vulnus-mac.app
open ./vulnus-mac.app