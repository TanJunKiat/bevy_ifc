use bevy::prelude::*;

use crate::IfcDirection;
use crate::IfcCartesianPoint;
use crate::IfcAxis2Placement3D;
use crate::IfcAxis2Placement2D;
use crate::IfcLocalPlacement;
use crate::IfcPolyLine;
use crate::IfcPolyLoop;
use crate::IfcFace;
use crate::IfcShapeRepresentation;
use crate::IfcRectangleProfileDef;
use crate::IfcPropertySingleValue;
use crate::IfcFaceOuterBound;
use crate::IfcPropertySet;
use crate::IfcRelDefinesByProperties;
use crate::IfcExtrudedAreaSolid;
use crate::IfcPlane;
use crate::IfcConnectionSurfaceGeometry;
use crate::IfcCompositeCurveSegment;
use crate::IfcArbitraryOpenProfileDef;
use crate::IfcConnectedFaceSet;
use crate::IfcMappedItem;
use crate::IfcWallStandardCase;
use crate::IfcCartesianTransformationOperator3D;
use crate::IfcStyledItem;
use crate::IfcProductDefinitionShape;
use crate::IfcSurfaceOfLinearExtrusion;
use crate::IfcPresentationStyleAssignment;
use crate::IfcMaterialLayerSetUsage;
use crate::IfcRelSpaceBoundary;
use crate::IfcRelAssociatesMaterial;
use crate::IfcRepresentationMap;
use crate::IfcCircle;
use crate::IfcTrimmedCurve;
use crate::IfcArbitraryClosedProfileDef;
use crate::IfcCurveStyleFontPattern;
use crate::IfcCurveBoundedPlane;
use crate::IfcFaceBound;
use crate::IfcColourRgb;
use crate::IfcCurveStyle;
use crate::IfcMaterialLayer;
use crate::IfcSlab;
use crate::IfcRelConnectsPathElements;
use crate::IfcFurnishingElement;
use crate::IfcOpeningElement;
use crate::IfcRelVoidsElement;
use crate::IfcCompositeCurve;
use crate::IfcDoor;
use crate::IfcGeometricSet;
use crate::IfcRelFillsElement;
use crate::IfcMaterial;
use crate::IfcArbitraryProfileDefWithVoids;
use crate::IfcWindow;
use crate::IfcFurnitureType;
use crate::IfcSpace;
use crate::IfcFaceBasedSurfaceModel;
use crate::IfcSurfaceStyleRendering;
use crate::IfcSurfaceStyle;
use crate::IfcWindowStyle;
use crate::IfcCurveStyleFont;
use crate::IfcRelDefinesByType;
use crate::IfcFillAreaStyleHatching;
use crate::IfcQuantityArea;
use crate::IfcElementQuantity;
use crate::IfcMaterialDefinitionRepresentation;
use crate::IfcStyledRepresentation;
use crate::IfcRelContainedInSpatialStructure;
use crate::IfcPresentationLayerAssignment;
use crate::IfcCovering;
use crate::IfcFillAreaStyle;
use crate::IfcMaterialLayerSet;
use crate::IfcPolygonalBoundedHalfSpace;
use crate::IfcWindowLiningProperties;
use crate::IfcBeam;
use crate::IfcCircleProfileDef;
use crate::IfcFooting;
use crate::IfcDoorStyle;
use crate::IfcRelAggregates;
use crate::IfcBooleanClippingResult;
use crate::IfcBuildingStorey;
use crate::IfcRailing;
use crate::IfcDoorLiningProperties;
use crate::IfcStair;
use crate::IfcStairFlight;




pub fn get_ifc_direction(
    ifc_direction_query: &Query<&mut IfcDirection,With<IfcDirection>>,
    idx: &String,
) -> IfcDirection
{
    for ifc_direction in ifc_direction_query.iter(){
        if &ifc_direction.idx == idx{
            return ifc_direction.clone();
        }
    }
    return IfcDirection::default();
}

pub fn get_ifc_cartesian_point(
    ifc_cartesian_point_query: &Query<&mut IfcCartesianPoint,With<IfcCartesianPoint>>,
    idx: &String,
) -> IfcCartesianPoint
{
    for ifc_cartesian_point in ifc_cartesian_point_query.iter(){
        if &ifc_cartesian_point.idx == idx{
            return ifc_cartesian_point.clone();
        }
    }
    return IfcCartesianPoint::default();
}

pub fn get_ifc_poly_line(
    ifc_poly_line_query: &Query<&mut IfcPolyLine,With<IfcPolyLine>>,
    idx: &String,
) -> IfcPolyLine
{
    for ifc_poly_line in ifc_poly_line_query.iter(){
        if &ifc_poly_line.idx == idx{
            return ifc_poly_line.clone();
        }
    }
    return IfcPolyLine::default();
}


pub fn get_ifc_axis_2_placement_3d(
    ifc_axis_2_placement_3d_query: &Query<&mut IfcAxis2Placement3D,With<IfcAxis2Placement3D>>,
    idx: &String,
) -> IfcAxis2Placement3D
{
    for ifc_axis_2_placement_3d in ifc_axis_2_placement_3d_query.iter(){
        if &ifc_axis_2_placement_3d.idx == idx{
            return ifc_axis_2_placement_3d.clone();
        }
    }
    return IfcAxis2Placement3D::default();
}

pub fn get_ifc_shape_representation(
    ifc_shape_representation_query: &Query<&mut IfcShapeRepresentation,With<IfcShapeRepresentation>>,
    idx: &String,
) -> IfcShapeRepresentation
{
    for ifc_shape_representation in ifc_shape_representation_query.iter(){
        if &ifc_shape_representation.idx == idx{
            return ifc_shape_representation.clone();
        }
    }
    return IfcShapeRepresentation::default();
}
















pub fn get_ifc_product_definition_shape(
    ifc_product_definition_shape_query: &Query<&mut IfcProductDefinitionShape,With<IfcProductDefinitionShape>>,
    idx: &String,
) -> IfcProductDefinitionShape
{
    for ifc_product_definition_shape in ifc_product_definition_shape_query.iter(){
        if &ifc_product_definition_shape.idx == idx{
            return ifc_product_definition_shape.clone();
        }
    }
    return IfcProductDefinitionShape::default();
}

pub fn get_ifc_shape_representation_shape(
    ifc_shape_representation_shape_query: &Query<&mut IfcShapeRepresentation,With<IfcShapeRepresentation>>,
    idx: &String,
) -> IfcShapeRepresentation
{
    for ifc_shape_representation_shape in ifc_shape_representation_shape_query.iter(){
        if &ifc_shape_representation_shape.idx == idx{
            return ifc_shape_representation_shape.clone();
        }
    }
    return IfcShapeRepresentation::default();
}


