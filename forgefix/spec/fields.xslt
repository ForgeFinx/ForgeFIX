<?xml version="1.0" encoding="UTF-8"?>
<xsl:stylesheet version="1.0" xmlns:xsl="http://www.w3.org/1999/XSL/Transform">
<xsl:output method="text" omit-xml-declaration="yes" />
<xsl:template match="/">
#![allow(non_camel_case_types, dead_code)]
#![allow(clippy::upper_case_acronyms)]

<xsl:apply-templates />
</xsl:template>

<xsl:template match="/fix/messages">
    pub fn is_session_message(msg_type: char) -> bool{
        matches!(msg_type,
   <xsl:for-each select="message[@msgcat='admin']">'<xsl:value-of select="@msgtype"/>' <xsl:if test="position() != last()"> <xsl:text>|</xsl:text> </xsl:if> </xsl:for-each>
        )
    }
</xsl:template>

<xsl:template match="/fix/fields">
#[repr(C)]
#[derive(Debug)]
pub enum Tags {
   <xsl:for-each select="field"><xsl:value-of select="@name"/> = <xsl:value-of select="@number"/>,</xsl:for-each>
}
impl TryFrom&lt;u32&gt; for Tags {
    type Error = &amp;'static str;
    fn try_from(u : u32) -> Result&lt;Self, Self::Error&gt; {
    match u {
    <xsl:for-each select="field"><xsl:value-of select="@number"/> => Ok(Tags::<xsl:value-of select="@name"/>),</xsl:for-each>
    _=> Err("illegal value for Tag")
        }
    }
}
impl From&lt;Tags&gt; for u32 {
    fn from(value: Tags) -> u32 {
        value as isize as u32
    }
}


pub fn get_data_ref(tag: u32) -> Option&lt;u32&gt; {
match tag {
<xsl:for-each select="./field[@type = 'DATA']">
    <xsl:variable name="siblingXpath" select="../field[@name = concat(current()/@name, 'Len') or @name = concat(current()/@name, 'Length')]" />
    <xsl:if test="$siblingXpath">
    <xsl:for-each select="$siblingXpath">
        <xsl:value-of select="@number"/><xsl:if test="position() != last()"> <xsl:text>|</xsl:text> </xsl:if>
    </xsl:for-each> => Some(<xsl:value-of select="@number"/>), 
    </xsl:if>
</xsl:for-each>
_=>None
}
}
<xsl:apply-templates/>
</xsl:template>

<xsl:template match="/fix/fields/field[@type = 'CHAR' or @type = 'BOOLEAN'][value]">
#[repr(C)]
#[derive(Debug,PartialEq,Eq)]
pub enum <xsl:value-of select="./@name" /> {
    <xsl:for-each select="value">
    <xsl:value-of select="./@description"/> = '<xsl:value-of select="./@enum"/>' as isize,
    </xsl:for-each>
}

impl From&lt;<xsl:value-of select="./@name" />&gt; for char {
    fn from(a:<xsl:value-of select="./@name" />) -> char {
        a as isize as u8 as char
    }
}
<xsl:if test="*">
impl From&lt;<xsl:value-of select="./@name" />&gt; for &amp;'static [u8] {
    fn from(a:<xsl:value-of select="./@name" />) -> &amp;'static [u8] {
        match a {
    <xsl:for-each select="value">
    <xsl:value-of select="../@name"/>::<xsl:value-of select="./@description"/> => b"<xsl:value-of select="./@enum"/>",
    </xsl:for-each>
        }
    }
}

impl TryFrom&lt;char&gt; for <xsl:value-of select="./@name" /> {
    type Error = &amp;'static str;
    fn try_from(c : char) -> Result&lt;Self, Self::Error&gt; {
    match c {
    <xsl:for-each select="value">'<xsl:value-of select="./@enum"/>' => Ok(Self::<xsl:value-of select="./@description"/>),</xsl:for-each>
    _=> Err("illegal value for field")
        }
    }
}
</xsl:if>
</xsl:template>

<xsl:template match="/fix/fields/field[@type = 'INT'][value]">
#[repr(C)]
#[derive(Debug,PartialEq,Eq)]
pub enum <xsl:value-of select="./@name" /> {
    <xsl:for-each select="value">
    <xsl:value-of select="./@description"/> = <xsl:value-of select="./@enum"/>,
    </xsl:for-each>
}
<xsl:if test="*">
impl TryFrom&lt;u8&gt; for <xsl:value-of select="./@name" /> {
    type Error = &amp;'static str;
    fn try_from(c : u8) -> Result&lt;Self, Self::Error&gt; {
    match c {
    <xsl:for-each select="value"><xsl:value-of select="./@enum"/> => Ok(Self::<xsl:value-of select="./@description"/>),</xsl:for-each>
    _=> Err("illegal value for field")
        }
    }
}
</xsl:if>
</xsl:template>

</xsl:stylesheet>
