
import copy

with open('samples/duplex.ifc') as f:
    lines = f.readlines()

lines_copy = copy.deepcopy(lines)
range_idx = range(len(lines))
# range_idx.reverse()
for idx in reversed(range_idx):
    if lines[idx].find("IFCCARTESIANPOINT(") != -1:
        lines_copy.pop(idx)
    elif lines[idx].find("IFCDIRECTION(") != -1:
        lines_copy.pop(idx)
    elif lines[idx].find("IFCAXIS2PLACEMENT3D(") != -1:
        lines_copy.pop(idx)
    elif lines[idx].find("IFCAXIS2PLACEMENT2D(") != -1:
        lines_copy.pop(idx)
    elif lines[idx].find("IFCLOCALPLACEMENT(") != -1:
        lines_copy.pop(idx)
    elif lines[idx].find("IFCPOLYLINE(") != -1:
        lines_copy.pop(idx)
    elif lines[idx].find("IFCPOLYLOOP(") != -1:
        lines_copy.pop(idx)
    elif lines[idx].find("IFCFACE(") != -1:
        lines_copy.pop(idx)
    elif lines[idx].find("IFCFACEOUTERBOUND(") != -1:
        lines_copy.pop(idx)
    elif lines[idx].find("IFCSHAPEREPRESENTATION(") != -1:
        lines_copy.pop(idx)
    elif lines[idx].find("IFCRECTANGLEPROFILEDEF(") != -1:
        lines_copy.pop(idx)
    elif lines[idx].find("IFCPROPERTYSINGLEVALUE(") != -1:
        lines_copy.pop(idx)
    elif lines[idx].find("IFCPROPERTYSET(") != -1:
        lines_copy.pop(idx)
    elif lines[idx].find("IFCRELDEFINESBYPROPERTIES(") != -1:
        lines_copy.pop(idx)
    elif lines[idx].find("IFCEXTRUDEDAREASOLID(") != -1:
        lines_copy.pop(idx)
    elif lines[idx].find("IFCPLANE(") != -1:
        lines_copy.pop(idx)
    elif lines[idx].find("IFCCONNECTIONSURFACEGEOMETRY(") != -1:
        lines_copy.pop(idx)
    elif lines[idx].find("IFCCOMPOSITECURVESEGMENT(") != -1:
        lines_copy.pop(idx)
    elif lines[idx].find("IFCARBITRARYOPENPROFILEDEF(") != -1:
        lines_copy.pop(idx)
    elif lines[idx].find("IFCCONNECTEDFACESET(") != -1:
        lines_copy.pop(idx)
    elif lines[idx].find("IFCMAPPEDITEM(") != -1:
        lines_copy.pop(idx)
    elif lines[idx].find("IFCWALLSTANDARDCASE(") != -1:
        lines_copy.pop(idx)
    elif lines[idx].find("IFCCARTESIANTRANSFORMATIONOPERATOR3D(") != -1:
        lines_copy.pop(idx)
    elif lines[idx].find("IFCSTYLEDITEM(") != -1:
        lines_copy.pop(idx)
    elif lines[idx].find("IFCPRODUCTDEFINITIONSHAPE(") != -1:
        lines_copy.pop(idx)
    elif lines[idx].find("IFCSURFACEOFLINEAREXTRUSION(") != -1:
        lines_copy.pop(idx)
    elif lines[idx].find("IFCPRESENTATIONSTYLEASSIGNMENT(") != -1:
        lines_copy.pop(idx)
    elif lines[idx].find("IFCMATERIALLAYERSETUSAGE(") != -1:
        lines_copy.pop(idx)
    elif lines[idx].find("IFCRELSPACEBOUNDARY(") != -1:
        lines_copy.pop(idx)
    elif lines[idx].find("IFCRELASSOCIATESMATERIAL(") != -1:
        lines_copy.pop(idx)
    elif lines[idx].find("IFCREPRESENTATIONMAP(") != -1:
        lines_copy.pop(idx)
    elif lines[idx].find("IFCCIRCLE(") != -1:
        lines_copy.pop(idx)
    elif lines[idx].find("IFCTRIMMEDCURVE(") != -1:
        lines_copy.pop(idx)
    elif lines[idx].find("IFCARBITRARYCLOSEDPROFILEDEF(") != -1:
        lines_copy.pop(idx)
    elif lines[idx].find("IFCCURVESTYLEFONTPATTERN(") != -1:
        lines_copy.pop(idx)
    elif lines[idx].find("IFCCURVEBOUNDEDPLANE(") != -1:
        lines_copy.pop(idx)
    elif lines[idx].find("IFCFACEBOUND(") != -1:
        lines_copy.pop(idx)
    elif lines[idx].find("IFCCOLOURRGB(") != -1:
        lines_copy.pop(idx)
    elif lines[idx].find("IFCCURVESTYLE(") != -1:
        lines_copy.pop(idx)
    elif lines[idx].find("IFCMATERIALLAYER(") != -1:
        lines_copy.pop(idx)
    elif lines[idx].find("IFCSLAB(") != -1:
        lines_copy.pop(idx)
    elif lines[idx].find("IFCDOOR(") != -1:
        lines_copy.pop(idx)
    elif lines[idx].find("IFCRELCONNECTSPATHELEMENTS(") != -1:
        lines_copy.pop(idx)
    elif lines[idx].find("IFCFURNISHINGELEMENT(") != -1:
        lines_copy.pop(idx)
    elif lines[idx].find("IFCOPENINGELEMENT(") != -1:
        lines_copy.pop(idx)
    elif lines[idx].find("IFCRELVOIDSELEMENT(") != -1:
        lines_copy.pop(idx)
    elif lines[idx].find("IFCCOMPOSITECURVE(") != -1:
        lines_copy.pop(idx)
    elif lines[idx].find("IFCGEOMETRICSET(") != -1:
        lines_copy.pop(idx)
    elif lines[idx].find("IFCRELFILLSELEMENT(") != -1:
        lines_copy.pop(idx)
    elif lines[idx].find("IFCMATERIAL(") != -1:
        lines_copy.pop(idx)
    elif lines[idx].find("IFCARBITRARYPROFILEDEFWITHVOIDS(") != -1:
        lines_copy.pop(idx)
    elif lines[idx].find("IFCWINDOW(") != -1:
        lines_copy.pop(idx)
    elif lines[idx].find("IFCFURNITURETYPE(") != -1:
        lines_copy.pop(idx)
    elif lines[idx].find("IFCSPACE(") != -1:
        lines_copy.pop(idx)
    elif lines[idx].find("IFCFACEBASEDSURFACEMODEL(") != -1:
        lines_copy.pop(idx)
    elif lines[idx].find("IFCSURFACESTYLERENDERING(") != -1:
        lines_copy.pop(idx)
    elif lines[idx].find("IFCSURFACESTYLE(") != -1:
        lines_copy.pop(idx)
    elif lines[idx].find("IFCWINDOWSTYLE(") != -1:
        lines_copy.pop(idx)
    elif lines[idx].find("IFCCURVESTYLEFONT(") != -1:
        lines_copy.pop(idx)
    elif lines[idx].find("IFCRELDEFINESBYTYPE(") != -1:
        lines_copy.pop(idx)
    elif lines[idx].find("IFCFILLAREASTYLEHATCHING(") != -1:
        lines_copy.pop(idx)
    elif lines[idx].find("IFCQUANTITYAREA(") != -1:
        lines_copy.pop(idx)
    elif lines[idx].find("IFCELEMENTQUANTITY(") != -1:
        lines_copy.pop(idx)
    elif lines[idx].find("IFCMATERIALDEFINITIONREPRESENTATION(") != -1:
        lines_copy.pop(idx)
    elif lines[idx].find("IFCSTYLEDREPRESENTATION(") != -1:
        lines_copy.pop(idx)
    elif lines[idx].find("IFCRELCONTAINEDINSPATIALSTRUCTURE(") != -1:
        lines_copy.pop(idx)
    elif lines[idx].find("IFCPRESENTATIONLAYERASSIGNMENT(") != -1:
        lines_copy.pop(idx)
    elif lines[idx].find("IFCCOVERING(") != -1:
        lines_copy.pop(idx)
    elif lines[idx].find("IFCFILLAREASTYLE(") != -1:
        lines_copy.pop(idx)
    elif lines[idx].find("IFCMATERIALLAYERSET(") != -1:
        lines_copy.pop(idx)
    elif lines[idx].find("IFCPOLYGONALBOUNDEDHALFSPACE(") != -1:
        lines_copy.pop(idx)
    elif lines[idx].find("IFCWINDOWLININGPROPERTIES(") != -1:
        lines_copy.pop(idx)
    elif lines[idx].find("IFCBEAM(") != -1:
        lines_copy.pop(idx)
    elif lines[idx].find("IFCCIRCLEPROFILEDEF(") != -1:
        lines_copy.pop(idx)
    elif lines[idx].find("IFCFOOTING(") != -1:
        lines_copy.pop(idx)
    elif lines[idx].find("IFCDOORSTYLE(") != -1:
        lines_copy.pop(idx)
    elif lines[idx].find("IFCRELAGGREGATES(") != -1:
        lines_copy.pop(idx)
    elif lines[idx].find("IFCBOOLEANCLIPPINGRESULT(") != -1:
        lines_copy.pop(idx)
    elif lines[idx].find("IFCBUILDINGSTOREY(") != -1:
        lines_copy.pop(idx)
    elif lines[idx].find("IFCRAILING(") != -1:
        lines_copy.pop(idx)
    elif lines[idx].find("IFCDOORLININGPROPERTIES(") != -1:
        lines_copy.pop(idx)
    elif lines[idx].find("IFCSTAIR(") != -1:
        lines_copy.pop(idx)
    elif lines[idx].find("IFCSTAIRFLIGHT(") != -1:
        lines_copy.pop(idx)

with open('test/python/filtered.ifc', 'w') as f:
    f.writelines(lines_copy)